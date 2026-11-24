use thiserror::Error;

use crate::circuit_prover::CircuitError;

/// The errors of proving.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Circuit(#[from] CircuitError),
    #[error("cannot write the inputs of {circuit}: {source}")]
    Inputs {
        circuit: String,
        source: toml::ser::Error,
    },
    #[error(
        "an action needs a membership path for a persistent consumed resource, and a root for an ephemeral one"
    )]
    Membership,
    #[error("the membership path has {length} siblings, more than the largest path length {max}")]
    PathTooLong { length: usize, max: usize },
    #[error("{circuit} has {count} public inputs, expected {expected}")]
    PublicInputCount {
        circuit: String,
        count: usize,
        expected: usize,
    },
    #[error("the logic ref {logic_ref} is not the key hash {key_hash} of {circuit}")]
    LogicRef {
        circuit: String,
        logic_ref: String,
        key_hash: String,
    },
    #[error("{circuit} cannot make its private inputs: {source}")]
    LogicInputs {
        circuit: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error(
        "the logic proof of the logic ref {logic_ref} does not carry a verification key that the verifier knows"
    )]
    UnknownLogic { logic_ref: String },
    #[error(
        "the transaction is not balanced: its delta is not the generator times the sum of its rcv values"
    )]
    Unbalanced,
    #[error("the transaction has no actions")]
    Empty,
    #[error(
        "an action has {resource_count} resources; the circuits take actions of 1 to {max} resources"
    )]
    ResourceCount { resource_count: usize, max: usize },
    #[error("an action of {resource_count} resources carries {logic_proof_count} logic proofs")]
    LogicProofCount {
        logic_proof_count: usize,
        resource_count: usize,
    },
    #[error(transparent)]
    Arm(#[from] anoma_arm::Error),
    #[error("cannot wrap the root proof: {0}")]
    Wrap(Box<dyn std::error::Error + Send + Sync>),
}
