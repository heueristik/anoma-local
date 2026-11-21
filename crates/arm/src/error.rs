use thiserror::Error;

/// The errors of the resource machine computations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("the nullifier key does not open the nullifier key commitment of the resource")]
    NullifierKeyMismatch,
    #[error("leaf {index} is not in the belt of {leaf_count} leaves")]
    LeafNotInBelt { index: usize, leaf_count: usize },
    #[error(
        "the path of leaf {index} has {length} nodes, more than the {max} that the circuits allow"
    )]
    PathTooLong {
        index: usize,
        length: usize,
        max: usize,
    },
    #[error("the {0} word of the resource plaintext is out of range")]
    InvalidWord(&'static str),
    #[error("{0} is not a BN254 scalar field element")]
    NotInField(String),
}
