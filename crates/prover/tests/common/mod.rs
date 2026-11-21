#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::OnceLock;

use anoma_arm::Fr;
use anoma_arm::belt::MembershipPath;
use anoma_arm::digest::AppData;
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_prover::action::{ActionWitness, Logic};
use anoma_prover::toolchain::Toolchain;
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

/// The directory for the Groth16 keys, which stays across runs.
pub fn wrap_keys_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys")
}

pub fn trivial_logic_ref() -> anyhow::Result<Fr> {
    Ok(toolchain()
        .verification_key(&Logic::trivial().circuit)
        .context("write the key of the trivial logic")?
        .hash)
}

pub fn nullifier_key(seed: u64) -> NullifierKey {
    NullifierKey(Fr::from(1000 + seed))
}

/// A resource of the trivial logic, owned by the nullifier key of the same seed.
pub fn trivial_resource(seed: u64, label: u64, quantity: u128) -> anyhow::Result<Resource> {
    Ok(Resource {
        logic_ref: trivial_logic_ref()?,
        label_ref: Fr::from(label),
        value_ref: Fr::from(0u64),
        quantity,
        nonce: Fr::from(2000 + seed),
        nullifier_key_commitment: nullifier_key(seed).commitment(),
        is_ephemeral: false,
        rand_seed: Fr::from(3000 + seed),
    })
}

/// An action that consumes an ephemeral resource and creates one of the same kind.
pub fn ephemeral_action(
    seed: u64,
    consumed_quantity: u128,
    created_quantity: u128,
) -> anyhow::Result<ActionWitness> {
    let consumed = Resource {
        is_ephemeral: true,
        ..trivial_resource(seed, 1, consumed_quantity)?
    };
    let nullifier = consumed.nullifier(nullifier_key(seed))?;
    let created = Resource {
        nonce: created_nonce(nullifier),
        ..trivial_resource(seed + 1, 1, created_quantity)?
    };
    Ok(ActionWitness {
        consumed,
        consumed_logic: Logic::trivial(),
        consumed_app_data: AppData::default(),
        nullifier_key: nullifier_key(seed),
        path: MembershipPath::default(),
        commitment_tree_root: Fr::from(0u64),
        created,
        created_logic: Logic::trivial(),
        created_app_data: AppData::default(),
        rcv: anoma_arm::Scalar::from(4000 + seed),
    })
}
