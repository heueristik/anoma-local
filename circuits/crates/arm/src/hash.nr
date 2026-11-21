use poseidon::poseidon2::Poseidon2;

pub global COMMITMENT: Field = 1;
pub global NULLIFIER: Field = 2;
pub global NULLIFIER_KEY: Field = 3;
pub global RCM: Field = 4;
pub global PSI: Field = 5;
pub global NONCE: Field = 6;
pub global KIND: Field = 7;
pub global ACTION_TREE: Field = 8;
pub global MMB_NODE: Field = 9;
pub global TERMS: Field = 10;
pub global PUBLIC_KEY: Field = 11;

/// Hashes the inputs with Poseidon2 after a domain tag, so that hashes for different purposes never share an input.
pub fn hash<let N: u32>(domain: Field, inputs: [Field; N]) -> Field {
    let mut preimage = [domain; N + 1];
    for i in 0..N {
        preimage[i + 1] = inputs[i];
    }
    Poseidon2::hash(preimage, N + 1)
}

#[test]
fn hash_differs_between_domains() {
    assert(hash(COMMITMENT, [1, 2]) != hash(NULLIFIER, [1, 2]));
}

#[test]
fn hash_puts_the_domain_before_the_inputs() {
    assert_eq(hash(MMB_NODE, [1, 2]), Poseidon2::hash([MMB_NODE, 1, 2], 3));
}
