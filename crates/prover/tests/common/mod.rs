#![allow(dead_code)]

use std::path::PathBuf;

use anoma_arm::belt::Belt;
use anoma_arm::resource::NullifierKey;
use anoma_arm::{Fr, Scalar};
use anoma_integration_test::fixtures::token;
use anoma_prover::action::{ActionWitness, Logic};

pub use anoma_integration_test::fixtures::toolchain;

/// The directory for the Groth16 keys, which stays across runs.
pub fn wrap_keys_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys")
}

/// An action that consumes an ephemeral token and creates one of the same kind.
pub fn ephemeral_action(
    seed: u64,
    consumed_quantity: u128,
    created_quantity: u128,
) -> anyhow::Result<ActionWitness> {
    let owner = NullifierKey(Fr::from(1000 + seed));
    Ok(ActionWitness::from_ephemeral(
        token(1, created_quantity, owner, 3000 + seed)?,
        consumed_quantity,
        owner,
        Logic::trivial(),
        Belt::default().root(),
        Scalar::from(4000 + seed),
    )?)
}
