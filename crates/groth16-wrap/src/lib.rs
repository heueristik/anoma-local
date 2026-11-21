//! Wraps a bb 6.0.0 UltraHonk proof (`noir-recursive-no-zk`) into a Groth16 proof over BN254 that the Solidity
//! verifier of gnark checks. The Go library in `go/` makes the proofs; this crate calls it through its C interface.
//! [`Groth16Wrapper`] is the wrapper of the root proof of the Anoma prover.

mod error;
mod ffi;
mod keys;
mod proof;
mod root_proof_wrapper;

use std::path::{Path, PathBuf};

pub use error::Error;
pub use keys::Keys;
pub use proof::Groth16Proof;
pub use root_proof_wrapper::Groth16Wrapper;

/// The length of one encoded field element: 32 bytes, big-endian.
pub const FIELD_BYTES: usize = 32;

/// Compiles the wrapper circuit for an inner verifying key, as bb writes it, and writes its keys to `directory`, which
/// must not exist. The keys go to a `.partial` folder first, so that a stopped setup leaves no `directory`.
///
/// The caller knows the setup secret and can make false proofs, so the keys are for tests only.
pub fn single_party_setup(inner_verifying_key: &[u8], directory: &Path) -> Result<Keys, Error> {
    let mut partial = directory.as_os_str().to_owned();
    partial.push(".partial");
    let partial = PathBuf::from(partial);
    if partial.exists() {
        std::fs::remove_dir_all(&partial).map_err(|source| Error::Io {
            path: partial.clone(),
            source,
        })?;
    }
    ffi::single_party_setup(inner_verifying_key, Keys::new(&partial).encoded_directory())?;
    std::fs::rename(&partial, directory).map_err(|source| Error::Io {
        path: directory.to_path_buf(),
        source,
    })?;
    Ok(Keys::new(directory))
}

/// Wraps an inner proof and its user public inputs, as bb writes them, into a Groth16 proof.
///
/// Returns [`Error::Rejected`] if the inner proof does not verify against the inner verifying key of `keys`.
pub fn prove(
    keys: &Keys,
    inner_proof: &[u8],
    public_inputs: &[[u8; FIELD_BYTES]],
) -> Result<Groth16Proof, Error> {
    let (bytes, encoded_inputs) = ffi::prove(
        keys.encoded_directory(),
        inner_proof,
        public_inputs.as_flattened(),
    )?;
    let (inputs, remainder) = encoded_inputs.as_chunks::<FIELD_BYTES>();
    if !remainder.is_empty() {
        return Err(Error::PublicInputsLength(encoded_inputs.len()));
    }
    Ok(Groth16Proof {
        bytes,
        public_inputs: inputs.to_vec(),
    })
}

/// Checks a Groth16 proof against its public inputs. Returns [`Error::Rejected`] if the proof is invalid.
pub fn verify(keys: &Keys, proof: &Groth16Proof) -> Result<(), Error> {
    ffi::verify(
        keys.encoded_directory(),
        &proof.bytes,
        proof.public_inputs.as_flattened(),
    )
}

/// Returns the source of the Solidity verifier for `keys`.
pub fn solidity_verifier(keys: &Keys) -> Result<String, Error> {
    Ok(String::from_utf8(ffi::solidity_verifier(
        keys.encoded_directory(),
    )?)?)
}
