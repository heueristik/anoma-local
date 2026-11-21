//! Parties, token resources and actions for the end-to-end tests.

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use anoma_arm::app_data::AppData;
use anoma_arm::belt::{Belt, MembershipPath};
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::{Discovery, GrumpkinAddress, GrumpkinDiscovery};
use anoma_groth16_wrap::Groth16Wrapper;
use anoma_intent::{IntentLogic, Offer, Payload, Terms};
use anoma_prover::action::{self, ActionWitness, Membership, ProvedAction};
use anoma_prover::circuit_prover::CircuitProver;
use anoma_prover::cli_prover::CliProver;
use anoma_prover::resource_logic::ResourceLogic;
use anoma_token::{TokenLogic, public_key_hash};
use anyhow::Context;
use p256::ecdsa::SigningKey;
use tempfile::TempDir;

/// The circuit prover on the compiled circuits, with one work directory for each test binary.
pub fn circuit_prover() -> &'static CliProver {
    static CIRCUIT_PROVER: OnceLock<(TempDir, CliProver)> = OnceLock::new();
    &CIRCUIT_PROVER
        .get_or_init(|| {
            let circuits = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../circuits");
            assert!(
                circuits.join("target/root.json").exists(),
                "the premise is that `just circuits-keys` compiled the circuits"
            );
            let work = TempDir::new().expect("a temporary directory");
            let prover = CliProver::from_env(circuits, work.path().to_path_buf());
            (work, prover)
        })
        .1
}

/// The Groth16 wrapper of the root proof, whose keys stay in `directory` across runs.
pub fn groth16_wrapper(directory: &Path) -> anyhow::Result<Groth16Wrapper> {
    Groth16Wrapper::single_party_setup(circuit_prover(), directory)
        .context("set up the Groth16 keys")
}

/// A party of the tests: the nullifier key and the P-256 key of the secret scalar `seed`. A party owns tokens, and an
/// issuer is a party too.
#[derive(Clone, Debug)]
pub struct Party {
    pub nullifier_key: NullifierKey,
    pub key: Arc<SigningKey>,
}

impl Party {
    pub fn new(seed: u64) -> Self {
        let mut secret = [0; 32];
        secret[24..].copy_from_slice(&seed.to_be_bytes());
        Self {
            nullifier_key: NullifierKey(Fr::from(seed)),
            key: Arc::new(
                SigningKey::from_slice(&secret)
                    .expect("a nonzero scalar below the group order is a secret key"),
            ),
        }
    }

    /// The value ref of the tokens of the party, and the label ref of the tokens that it issues.
    pub fn key_hash(&self) -> Fr {
        public_key_hash(self.key.verifying_key())
    }

    /// The token logic that signs with the key of the party.
    pub fn token_logic(&self) -> Arc<dyn ResourceLogic> {
        Arc::new(TokenLogic {
            signer: Some(self.key.clone()),
        })
    }
}

/// The logic ref of the token logic.
pub fn token_logic_ref() -> anyhow::Result<Fr> {
    Ok(circuit_prover()
        .verification_key(anoma_token::CIRCUIT)
        .context("write the key of the token logic")?
        .hash)
}

/// The logic ref of the intent logic.
pub fn intent_logic_ref() -> anyhow::Result<Fr> {
    Ok(circuit_prover()
        .verification_key(anoma_intent::CIRCUIT)
        .context("write the key of the intent logic")?
        .hash)
}

/// A persistent token of `issuer`, owned by `owner`. The action that creates it sets its nonce.
pub fn token(
    issuer: &Party,
    quantity: u128,
    owner: &Party,
    rand_seed: u64,
) -> anyhow::Result<Resource> {
    Ok(Resource {
        logic_ref: token_logic_ref()?,
        label_ref: issuer.key_hash(),
        value_ref: owner.key_hash(),
        quantity,
        nonce: Fr::from(0u64),
        nullifier_key_commitment: owner.nullifier_key.commitment(),
        is_ephemeral: false,
        rand_seed: Fr::from(rand_seed),
    })
}

/// The terms of an intent that wants `want_quantity` tokens of `want_issuer` with `rand_seed` for `receiver`.
pub fn terms(
    want_issuer: &Party,
    want_quantity: u128,
    receiver: &Party,
    rand_seed: u64,
) -> anyhow::Result<Terms> {
    Ok(Terms {
        want_logic_ref: token_logic_ref()?,
        want_label_ref: want_issuer.key_hash(),
        want_value_ref: receiver.key_hash(),
        want_quantity,
        receiver_nullifier_key_commitment: receiver.nullifier_key.commitment(),
        want_rand_seed: Fr::from(rand_seed),
    })
}

/// An action that consumes `consumed` and creates `created` with the nonce that the nullifier of `consumed` gives.
pub fn action(
    consumed: Resource,
    consumed_logic: Arc<dyn ResourceLogic>,
    nullifier_key: NullifierKey,
    membership: Membership,
    created: Resource,
    created_logic: Arc<dyn ResourceLogic>,
    rcv: u64,
) -> anyhow::Result<ActionWitness> {
    let nullifier = consumed.nullifier(nullifier_key)?;
    Ok(ActionWitness {
        consumed,
        consumed_logic,
        consumed_app_data: AppData::default(),
        nullifier_key,
        membership,
        created: Resource {
            nonce: created_nonce(nullifier),
            ..created
        },
        created_logic,
        created_app_data: AppData::default(),
        rcv: Scalar::from(rcv),
    })
}

/// An action on the empty commitment tree that creates `created` from an ephemeral token of its kind and quantity, which
/// `issuer` signs.
pub fn mint(issuer: &Party, created: Resource, rcv: u64) -> anyhow::Result<ActionWitness> {
    Ok(ActionWitness::from_ephemeral(
        created,
        created.quantity,
        NullifierKey(Fr::from(rcv)),
        issuer.token_logic(),
        Belt::default().root(),
        Scalar::from(rcv),
    )?)
}

/// The proved action of `owner` that gives up `token` for an intent with `terms`. The intent carries the payload for the
/// solver, encrypted to its keys; `seed` makes the intent key and the rcv of the action.
pub fn propose(
    token: Resource,
    owner: &Party,
    path: MembershipPath,
    terms: Terms,
    receiver: GrumpkinAddress,
    solver: &GrumpkinAddress,
    seed: u64,
) -> anyhow::Result<Vec<ProvedAction>> {
    let payload = Payload {
        nonce: created_nonce(token.nullifier(owner.nullifier_key)?),
        rand_seed: Fr::from(seed),
        nullifier_key: NullifierKey(Fr::from(seed)),
        terms,
        offer: Offer {
            logic_ref: token.logic_ref,
            label_ref: token.label_ref,
            quantity: token.quantity,
        },
        rcv_total: Scalar::from(seed),
        receiver,
    };
    let mut witness = self::action(
        token,
        owner.token_logic(),
        owner.nullifier_key,
        Membership::Persistent(path),
        payload.intent(intent_logic_ref()?),
        Arc::new(IntentLogic { terms }),
        seed,
    )?;
    witness.created_app_data = GrumpkinDiscovery.app_data(solver, &payload.to_fields())?;
    Ok(action::prove(circuit_prover(), &witness)?.actions)
}
