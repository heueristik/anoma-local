#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use anoma_arm::belt::Belt;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_prover::action::ActionWitness;
use anoma_prover::circuit_prover::CircuitProver;
use anoma_prover::resource_logic::{ResourceLogic, TrivialLogic};

pub use anoma_integration_test::fixtures::circuit_prover;

/// The directory for the Groth16 keys, which stays across runs.
pub fn wrap_keys_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys")
}

/// An action that consumes an ephemeral resource of the trivial logic and creates one of the same kind.
pub fn ephemeral_action(
    seed: u64,
    consumed_quantity: u128,
    created_quantity: u128,
) -> anyhow::Result<ActionWitness> {
    let owner = NullifierKey(Fr::from(1000 + seed));
    Ok(ActionWitness::from_ephemeral(
        Resource {
            logic_ref: circuit_prover()
                .verification_key(TrivialLogic.circuit())?
                .hash,
            label_ref: Fr::from(1u64),
            quantity: created_quantity,
            nullifier_key_commitment: owner.commitment(),
            rand_seed: Fr::from(3000 + seed),
            ..Resource::default()
        },
        consumed_quantity,
        owner,
        Arc::new(TrivialLogic),
        Belt::default().root(),
        Scalar::from(4000 + seed),
    )?)
}
