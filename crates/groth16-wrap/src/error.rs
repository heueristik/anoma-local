use std::path::PathBuf;
use std::string::FromUtf8Error;

/// An error of the wrap.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A proof or its public inputs are invalid.
    #[error("the wrap rejected the input: {0}")]
    Rejected(String),
    /// The wrap failed for a reason other than an invalid input, for example a missing key file.
    #[error("the wrap failed: {0}")]
    Failed(String),
    /// The Go library returned a status that this crate does not define.
    #[error("the wrap returned the unknown status {0}")]
    UnknownStatus(i32),
    /// The Go library returned public inputs whose length is not a multiple of 32 bytes.
    #[error("the wrap returned {0} bytes of public inputs, which is not a multiple of 32")]
    PublicInputsLength(usize),
    /// The setup cannot remove, write or rename a key folder.
    #[error("cannot set up the keys in {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    /// The Go library returned a Solidity verifier that is not UTF-8.
    #[error("the Solidity verifier is not UTF-8: {0}")]
    SolidityVerifierEncoding(#[from] FromUtf8Error),
}
