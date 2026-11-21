use crate::FIELD_BYTES;

/// A Groth16 proof in the argument layout of `verifyProof(bytes proof, uint256[n] input)` of the Solidity verifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Groth16Proof {
    /// The `proof` argument: A, B and C in the EIP-197 layout, then the commitments and their proof of knowledge.
    pub bytes: Vec<u8>,
    /// The `input` argument: the user public inputs of the inner proof, in the same order.
    pub public_inputs: Vec<[u8; FIELD_BYTES]>,
}
