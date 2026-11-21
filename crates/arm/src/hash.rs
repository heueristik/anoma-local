//! The Poseidon2 hashes of the resource machine, each with a domain before its inputs, as in the Noir library.

use acir_field::GenericFieldElement;
use ark_ff::AdditiveGroup;
use bn254_blackbox_solver::poseidon2_permutation;

use crate::Fr;

/// The domain of a hash, which separates the hashes for different purposes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    Commitment = 1,
    Nullifier = 2,
    NullifierKey = 3,
    Rcm = 4,
    Psi = 5,
    Nonce = 6,
    Kind = 7,
    ActionTree = 8,
    BeltNode = 9,
    Terms = 10,
}

/// Hashes the domain and the inputs with Poseidon2.
pub fn hash(domain: Domain, inputs: &[Fr]) -> Fr {
    let mut preimage = Vec::with_capacity(inputs.len() + 1);
    preimage.push(Fr::from(domain as u64));
    preimage.extend_from_slice(inputs);
    poseidon2_hash(&preimage)
}

/// The Poseidon2 sponge of Noir's `Poseidon2::hash`: rate 3, with the input length times 2^64 as the initial capacity.
pub fn poseidon2_hash(inputs: &[Fr]) -> Fr {
    let mut state = [
        Fr::ZERO,
        Fr::ZERO,
        Fr::ZERO,
        Fr::from(inputs.len() as u64) * Fr::from(1u128 << 64),
    ];
    for chunk in inputs.chunks(3) {
        for (lane, input) in state.iter_mut().zip(chunk) {
            *lane += input;
        }
        state = permute(state);
    }
    if inputs.is_empty() {
        state = permute(state);
    }
    state[0]
}

fn permute(state: [Fr; 4]) -> [Fr; 4] {
    let output = poseidon2_permutation(&state.map(GenericFieldElement::from_repr))
        .expect("the permutation takes a state of four elements");
    [0, 1, 2, 3].map(|lane| output[lane].into_repr())
}
