//! Checks the wrap of the `one_input` fixture from the setup to the native Groth16 check. Behind `slow-tests`: the
//! single-party setup takes minutes and up to 13 GB of memory, and it writes 1.8 GB of keys.

#![cfg(feature = "slow-tests")]

mod common;

use std::fs;
use std::sync::OnceLock;

use anoma_groth16_wrap::{
    Error, FIELD_BYTES, Groth16Proof, WrapKeys, prove, single_party_setup, solidity_verifier,
    verify,
};
use anyhow::{Context, anyhow};

/// The position of the first sumcheck field in a bb 6.0.0 proof: after 8 pairing point limbs and 8 commitments of 4 limbs.
const FIRST_SUMCHECK_FIELD: usize = 8 + 8 * 4;

struct Wrap {
    keys: WrapKeys,
    inner_proof: Vec<u8>,
    inner_public_inputs: Vec<[u8; FIELD_BYTES]>,
    proof: Groth16Proof,
}

/// Sets up the keys and wraps the fixture proof one time for all tests of this binary.
fn wrap() -> anyhow::Result<&'static Wrap> {
    static WRAP: OnceLock<Result<Wrap, String>> = OnceLock::new();
    WRAP.get_or_init(|| make_wrap().map_err(|error| format!("{error:#}")))
        .as_ref()
        .map_err(|error| anyhow!("{error}"))
}

fn make_wrap() -> anyhow::Result<Wrap> {
    let directory = common::keys_directory("keys-of-one-input");
    if directory.exists() {
        fs::remove_dir_all(&directory)
            .with_context(|| format!("remove the old keys in {}", directory.display()))?;
    }
    let keys =
        single_party_setup(&common::fixture("vk")?, &directory).context("set up the keys")?;
    let inner_proof = common::fixture("proof")?;
    let inner_public_inputs = public_inputs(&common::fixture("public_inputs")?)?;
    let proof =
        prove(&keys, &inner_proof, &inner_public_inputs).context("wrap the fixture proof")?;
    Ok(Wrap {
        keys,
        inner_proof,
        inner_public_inputs,
        proof,
    })
}

fn public_inputs(data: &[u8]) -> anyhow::Result<Vec<[u8; FIELD_BYTES]>> {
    let (inputs, remainder) = data.as_chunks::<FIELD_BYTES>();
    anyhow::ensure!(
        remainder.is_empty(),
        "the public inputs file has {} bytes, not a multiple of {FIELD_BYTES}",
        data.len()
    );
    Ok(inputs.to_vec())
}

#[test]
fn verify_accepts_the_wrap_of_a_valid_proof() -> anyhow::Result<()> {
    let wrap = wrap()?;
    verify(&wrap.keys, &wrap.proof).context("verify the wrap")
}

#[test]
fn prove_gives_the_inner_public_input_as_the_only_public_input() -> anyhow::Result<()> {
    let wrap = wrap()?;
    assert_eq!(
        wrap.inner_public_inputs.len(),
        1,
        "the premise is that the fixture has one public input"
    );
    assert_eq!(wrap.proof.public_inputs, wrap.inner_public_inputs);
    Ok(())
}

#[test]
fn verify_rejects_a_proof_with_any_flipped_byte() -> anyhow::Result<()> {
    let wrap = wrap()?;
    for position in 0..wrap.proof.bytes.len() {
        let mut proof = wrap.proof.clone();
        proof.bytes[position] ^= 0xff;
        let result = verify(&wrap.keys, &proof);
        assert!(
            matches!(result, Err(Error::Rejected(_))),
            "byte {position}: expected a rejection, got {result:?}"
        );
    }
    Ok(())
}

#[test]
fn verify_rejects_a_wrong_public_input() -> anyhow::Result<()> {
    let wrap = wrap()?;
    let mut proof = wrap.proof.clone();
    proof.public_inputs[0][FIELD_BYTES - 1] ^= 0xff;
    let result = verify(&wrap.keys, &proof);
    assert!(
        matches!(result, Err(Error::Rejected(_))),
        "expected a rejection, got {result:?}"
    );
    Ok(())
}

#[test]
fn prove_rejects_an_inner_proof_with_a_flipped_byte() -> anyhow::Result<()> {
    let wrap = wrap()?;
    let mut inner_proof = wrap.inner_proof.clone();
    inner_proof[FIRST_SUMCHECK_FIELD * FIELD_BYTES + FIELD_BYTES - 1] ^= 0xff;
    let result = prove(&wrap.keys, &inner_proof, &wrap.inner_public_inputs);
    let Err(Error::Rejected(message)) = result else {
        anyhow::bail!("expected a rejection, got {result:?}");
    };
    assert!(
        message.contains("the inner proof does not verify"),
        "unexpected message {message:?}"
    );
    Ok(())
}

#[test]
fn solidity_verifier_takes_one_public_input() -> anyhow::Result<()> {
    let wrap = wrap()?;
    let source = solidity_verifier(&wrap.keys)?;
    let signature = "function verifyProof(\n        bytes calldata proof,\n        uint256[1] calldata input\n    ) public view";
    assert!(
        source.contains(signature),
        "the verifier has no function {signature:?}"
    );
    Ok(())
}
