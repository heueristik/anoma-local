/// The number of bytes that the contract hashes for one action: the digest before it and the seven fields of the action.
global ACTION_BYTES: u32 = 8 * 32;

/// The app data hash of a resource with four empty payloads: the low 253 bits of the SHA-256 hash of four 4-byte zero counts.
pub global EMPTY_APP_DATA_HASH: Field =
    0x174708fff7719dd5979ec875d56cd2286f6d3cf7ec317a3b25632aab28ec37bb;

/// The public data of one action, in the order of the contract's `Action`, with each app data replaced by its hash.
pub struct ActionInstance {
    pub nullifier: Field,
    pub consumed_logic_ref: Field,
    pub consumed_app_data_hash: Field,
    pub commitment: Field,
    pub created_logic_ref: Field,
    pub created_app_data_hash: Field,
    pub commitment_tree_root: Field,
}

/// Extends the action digest with an action, as the contract does: the low 253 bits of the SHA-256 hash of the digest and the fields of the action, each as 32 big-endian bytes.
pub fn extend(digest: Field, action: ActionInstance) -> Field {
    let bytes: [u8; ACTION_BYTES] = be_bytes([
        digest,
        action.nullifier,
        action.consumed_logic_ref,
        action.consumed_app_data_hash,
        action.commitment,
        action.created_logic_ref,
        action.created_app_data_hash,
        action.commitment_tree_root,
    ]);
    low_253_bits(sha256::digest(bytes))
}

/// Binds the action digest to one chain and one AnomaVerifier, as the contract does: the low 253 bits of the SHA-256 hash of the chain ID, the address of the verifier and the digest, each as 32 big-endian bytes.
pub fn bind(chain_id: Field, verifier: Field, digest: Field) -> Field {
    let bytes: [u8; 96] = be_bytes([chain_id, verifier, digest]);
    low_253_bits(sha256::digest(bytes))
}

/// The fields, each as 32 big-endian bytes, one after the other.
fn be_bytes<let N: u32, let M: u32>(fields: [Field; N]) -> [u8; M] {
    assert_eq(M, 32 * N, "the byte array does not hold 32 bytes for each field");
    let mut bytes = [0; M];
    for i in 0..N {
        let field_bytes: [u8; 32] = fields[i].to_be_bytes();
        for j in 0..32 {
            bytes[32 * i + j] = field_bytes[j];
        }
    }
    bytes
}

/// Keeps the low 253 bits of a hash, so that the result is a BN254 field element.
pub fn low_253_bits(hash: [u8; 32]) -> Field {
    let mut bytes = hash;
    bytes[0] &= 0x1f;
    Field::from_be_bytes(bytes)
}

#[test]
fn low_253_bits_clears_the_top_three_bits() {
    let mut hash = [0; 32];
    hash[0] = 0xff;
    hash[31] = 1;
    assert_eq(low_253_bits(hash), 0x1f * 2.pow_32(248) + 1);
}

#[test]
fn extend_hashes_the_digest_and_the_action_fields_in_order() {
    let action = ActionInstance {
        nullifier: 1,
        consumed_logic_ref: 2,
        consumed_app_data_hash: 3,
        commitment: 4,
        created_logic_ref: 5,
        created_app_data_hash: 6,
        commitment_tree_root: 7,
    };
    let mut bytes = [0; ACTION_BYTES];
    for i in 0..8 {
        bytes[32 * i + 31] = i as u8;
    }
    assert_eq(extend(0, action), low_253_bits(sha256::digest(bytes)));
}

#[test]
fn empty_app_data_hash_is_the_hash_of_four_zero_counts() {
    assert_eq(EMPTY_APP_DATA_HASH, low_253_bits(sha256::digest([0; 16])));
}

#[test]
fn bind_matches_the_contract() {
    // The value of `ActionDigest.bind` for the digest 3, the chain ID 1 and the verifier at the address 2.
    assert_eq(bind(1, 2, 3), 0x1701f34c80e1ef7f8125e5d4d2d7e19b509e25d26e462d5308b5abb95b64783e);
}

#[test]
fn extend_matches_the_contract() {
    // The digest that the contract passes to the verifier for this action without blobs, on the empty commitment tree.
    let action = ActionInstance {
        nullifier: 1,
        consumed_logic_ref: 2,
        consumed_app_data_hash: EMPTY_APP_DATA_HASH,
        commitment: 4,
        created_logic_ref: 5,
        created_app_data_hash: EMPTY_APP_DATA_HASH,
        commitment_tree_root: 0,
    };
    assert_eq(
        extend(0, action),
        0x10545fff186a4bd171fe33842768c98e12133b06c13ee14b99182ef580ef78bf,
    );
}
