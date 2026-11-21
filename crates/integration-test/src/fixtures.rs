//! Parties, token resources and actions for the end-to-end tests.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anoma_arm::belt::MembershipPath;
use anoma_arm::digest::AppData;
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::Receiver;
use anoma_groth16_wrap::Keys;
use anoma_intent::{Offer, Payload, Terms};
use anoma_prover::action::{self, ActionWitness, Logic, ProvedAction};
use anoma_prover::toolchain::Toolchain;
use anoma_prover::transaction;
use anyhow::Context;
use tempfile::TempDir;

/// The toolchain on the compiled circuits, with one work directory for each test binary.
pub fn toolchain() -> &'static Toolchain {
    static TOOLCHAIN: OnceLock<(TempDir, Toolchain)> = OnceLock::new();
    &TOOLCHAIN
        .get_or_init(|| {
            let circuits = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../circuits");
            assert!(
                circuits.join("target/root.json").exists(),
                "the premise is that `just circuits-keys` compiled the circuits"
            );
            let work = TempDir::new().expect("a temporary directory");
            let toolchain = Toolchain::from_env(circuits, work.path().to_path_buf());
            (work, toolchain)
        })
        .1
}

/// The Groth16 keys of the root circuit, which stay in `directory` across runs.
pub fn wrap_keys(directory: &Path) -> anyhow::Result<Keys> {
    transaction::single_party_wrap_keys(toolchain(), directory).context("set up the Groth16 keys")
}

/// The logic ref of the trivial logic, which the token resources of the tests use.
pub fn token_logic_ref() -> anyhow::Result<Fr> {
    Ok(toolchain()
        .verification_key(&Logic::trivial().circuit)
        .context("write the key of the trivial logic")?
        .hash)
}

/// The logic ref of the intent logic.
pub fn intent_logic_ref() -> anyhow::Result<Fr> {
    Ok(toolchain()
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

/// Where a consumed resource is: in the commitment tree with the root, or nowhere for an ephemeral resource.
pub enum Membership {
    Tree { path: MembershipPath, root: Fr },
    Ephemeral,
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
    let (path, commitment_tree_root) = match membership {
        Membership::Tree { path, root } => (path, root),
        // The root of the empty tree is a historical root of every AnomaVerifier.
        Membership::Ephemeral => (MembershipPath::default(), Fr::from(0u64)),
    };
    Ok(ActionWitness {
        consumed,
        consumed_logic,
        consumed_app_data: AppData::default(),
        nullifier_key,
        path,
        commitment_tree_root,
        created: Resource {
            nonce: created_nonce(nullifier),
            ..created
        },
        created_logic,
        created_app_data: AppData::default(),
        rcv: Scalar::from(rcv),
    })
}

/// An action that creates `created` from an ephemeral resource of its kind and quantity, which anyone can make with
/// the trivial logic.
pub fn mint(created: Resource, rcv: u64) -> anyhow::Result<ActionWitness> {
    let source = NullifierKey(Fr::from(rcv));
    let consumed = Resource {
        nullifier_key_commitment: source.commitment(),
        is_ephemeral: true,
        ..created
    };
    action(
        consumed,
        Logic::trivial(),
        source,
        Membership::Ephemeral,
        created,
        Logic::trivial(),
        rcv,
    )
}

/// The proved action of a party that gives up `token` for an intent with `terms`. The intent carries the payload for
/// the solver, encrypted to its keys; `seed` makes the intent key and the rcv of the action.
pub fn propose(
    token: Resource,
    owner: NullifierKey,
    membership: Membership,
    terms: Terms,
    receiver: Receiver,
    solver: &Receiver,
    seed: u64,
) -> anyhow::Result<Vec<ProvedAction>> {
    let intent_key = NullifierKey(Fr::from(seed));
    let nonce = created_nonce(token.nullifier(owner)?);
    let intent = terms.intent(intent_logic_ref()?, intent_key, nonce, Fr::from(seed));
    let payload = Payload {
        intent,
        nullifier_key: intent_key,
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
        membership,
        intent,
        anoma_intent::created_logic()?,
        seed,
    )?;
    witness.created_app_data = anoma_discovery::app_data(solver, &payload.to_bytes())?;
    Ok(action::prove(toolchain(), &witness)?.actions)
}
