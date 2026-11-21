//! Parties, token resources and actions for the end-to-end tests.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anoma_arm::app_data::AppData;
use anoma_arm::belt::{Belt, MembershipPath};
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::{Discovery, EciesAddress, EciesDiscovery};
use anoma_groth16_wrap::Groth16Wrapper;
use anoma_intent::{Offer, Payload, Terms};
use anoma_prover::action::{self, ActionWitness, Logic, Membership, ProvedAction};
use anoma_prover::circuit_prover::CircuitProver;
use anoma_prover::cli_prover::CliProver;
use anyhow::Context;
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

/// The logic ref of the trivial logic, which the token resources of the tests use.
pub fn token_logic_ref() -> anyhow::Result<Fr> {
    Ok(circuit_prover()
        .verification_key(&Logic::trivial().circuit)
        .context("write the key of the trivial logic")?
        .hash)
}

/// The logic ref of the intent logic.
pub fn intent_logic_ref() -> anyhow::Result<Fr> {
    Ok(circuit_prover()
        .verification_key(anoma_intent::CIRCUIT)
        .context("write the key of the intent logic")?
        .hash)
}

/// A persistent token resource of the kind `label`, owned by `owner`. The action that creates it sets its nonce.
pub fn token(
    label: u64,
    quantity: u128,
    owner: NullifierKey,
    rand_seed: u64,
) -> anyhow::Result<Resource> {
    Ok(Resource {
        logic_ref: token_logic_ref()?,
        label_ref: Fr::from(label),
        value_ref: Fr::from(0u64),
        quantity,
        nonce: Fr::from(0u64),
        nullifier_key_commitment: owner.commitment(),
        is_ephemeral: false,
        rand_seed: Fr::from(rand_seed),
    })
}

/// The terms of an intent that wants the token of the kind `want_label` with `want_quantity` and `rand_seed` for
/// `receiver`.
pub fn terms(
    want_label: u64,
    want_quantity: u128,
    receiver: NullifierKey,
    rand_seed: u64,
) -> anyhow::Result<Terms> {
    Ok(Terms {
        want_logic_ref: token_logic_ref()?,
        want_label_ref: Fr::from(want_label),
        want_value_ref: Fr::from(0u64),
        want_quantity,
        receiver_nullifier_key_commitment: receiver.commitment(),
        want_rand_seed: Fr::from(rand_seed),
    })
}

/// An action that consumes `consumed` and creates `created` with the nonce that the nullifier of `consumed` gives.
pub fn action(
    consumed: Resource,
    consumed_logic: Logic,
    nullifier_key: NullifierKey,
    membership: Membership,
    created: Resource,
    created_logic: Logic,
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

/// An action on the empty commitment tree that creates `created` from an ephemeral resource of its kind and quantity,
/// which anyone can make with the trivial logic.
pub fn mint(created: Resource, rcv: u64) -> anyhow::Result<ActionWitness> {
    Ok(ActionWitness::from_ephemeral(
        created,
        created.quantity,
        NullifierKey(Fr::from(rcv)),
        Logic::trivial(),
        Belt::default().root(),
        Scalar::from(rcv),
    )?)
}

/// The proved action of a party that gives up `token` for an intent with `terms`. The intent carries the payload for
/// the solver, encrypted to its keys; `seed` makes the intent key and the rcv of the action.
pub fn propose(
    token: Resource,
    owner: NullifierKey,
    path: MembershipPath,
    terms: Terms,
    receiver: EciesAddress,
    solver: &EciesAddress,
    seed: u64,
) -> anyhow::Result<Vec<ProvedAction>> {
    let payload = Payload {
        nonce: created_nonce(token.nullifier(owner)?),
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
        Logic::trivial(),
        owner,
        Membership::Persistent(path),
        payload.intent(intent_logic_ref()?),
        anoma_intent::created_logic()?,
        seed,
    )?;
    witness.created_app_data = EciesDiscovery.app_data(solver, &payload.to_bytes())?;
    Ok(action::prove(circuit_prover(), &witness)?.actions)
}
