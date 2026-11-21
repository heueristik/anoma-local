use std::path::PathBuf;

use thiserror::Error;

/// The errors of proving.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{command} failed for {circuit}: {output}")]
    Tool {
        command: &'static str,
        circuit: String,
        output: String,
    },
    #[error("cannot run {program}: {source}")]
    Spawn {
        program: PathBuf,
        source: std::io::Error,
    },
    #[error("cannot read or write {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path} is not a bb file of the expected shape: {reason}")]
    Format { path: PathBuf, reason: String },
    #[error("cannot write the inputs of {circuit}: {source}")]
    Inputs {
        circuit: String,
        source: toml::ser::Error,
    },
    #[error("an action needs a membership path exactly when its consumed resource is persistent")]
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
    #[error(
        "the transaction is not balanced: its delta is not the generator times the sum of its rcv values"
    )]
    Unbalanced,
    #[error("the transaction has no actions")]
    Empty,
    #[error(transparent)]
    Arm(#[from] anoma_arm::Error),
    #[error(transparent)]
    Wrap(#[from] anoma_groth16_wrap::Error),
}
