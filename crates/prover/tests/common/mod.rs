#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use anoma_arm::belt::Belt;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_prover::action::{ActionWitness, Membership, ResourceWitness};
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
    split_action(seed, consumed_quantity, &[created_quantity])
}

/// An action that consumes an ephemeral resource of the trivial logic and creates resources of the same kind with
/// `created_quantities`.
pub fn split_action(
    seed: u64,
    consumed_quantity: u128,
    created_quantities: &[u128],
) -> anyhow::Result<ActionWitness> {
    let owner = NullifierKey(Fr::from(1000 + seed));
    let resource = Resource {
        logic_ref: circuit_prover()
            .verification_key(TrivialLogic.circuit())?
            .hash,
        label_ref: Fr::from(1u64),
        nullifier_key_commitment: owner.commitment(),
        rand_seed: Fr::from(3000 + seed),
        ..Resource::default()
    };
    let consumed = ResourceWitness::consumed(
        Resource {
            quantity: consumed_quantity,
            is_ephemeral: true,
            ..resource
        },
        Arc::new(TrivialLogic),
        owner,
        Membership::Ephemeral {
            root: Belt::default().root(),
        },
    );
    let created = created_quantities.iter().map(|&quantity| {
        ResourceWitness::created(
            Resource {
                quantity,
                ..resource
            },
            Arc::new(TrivialLogic),
        )
    });
    Ok(ActionWitness::new(
        std::iter::once(consumed).chain(created).collect(),
        Scalar::from(4000 + seed),
    )?)
}
