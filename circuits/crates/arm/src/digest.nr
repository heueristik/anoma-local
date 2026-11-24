/// The number of fields that the contract hashes for one resource of an action.
global RESOURCE_FIELDS: u32 = 5;

/// The app data hash of a resource with four empty payloads: the low 253 bits of the SHA-256 hash of four 4-byte zero counts.
pub global EMPTY_APP_DATA_HASH: Field =
    0x174708fff7719dd5979ec875d56cd2286f6d3cf7ec317a3b25632aab28ec37bb;

/// The public data of one resource of an action, in the order of the contract's `Resource`, with its app data replaced
/// by its hash. A created resource has the commitment tree root zero.
pub struct ResourceInstance {
    pub is_consumed: bool,
    pub tag: Field,
    pub logic_ref: Field,
    pub commitment_tree_root: Field,
    pub app_data_hash: Field,
}

/// Extends the action digest with an action, as the contract does: the low 253 bits of the SHA-256 hash of the digest
/// and the fields of each resource in the order of the list, each as 32 big-endian bytes.
pub fn extend<let K: u32>(digest: Field, resources: [ResourceInstance; K]) -> Field {
    let mut fields = [digest; 1 + RESOURCE_FIELDS * K];
    for i in 0..K {
        let resource = resources[i];
        let first = 1 + RESOURCE_FIELDS * i;
        fields[first] = resource.is_consumed as Field;
        fields[first + 1] = resource.tag;
        fields[first + 2] = resource.logic_ref;
        fields[first + 3] = resource.commitment_tree_root;
        fields[first + 4] = resource.app_data_hash;
    }
    let bytes: [u8; 32 * (1 + RESOURCE_FIELDS * K)] = be_bytes(fields);
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
fn extend_hashes_the_digest_and_the_resource_fields_in_order() {
    let resource = ResourceInstance {
        is_consumed: true,
        tag: 2,
        logic_ref: 3,
        commitment_tree_root: 4,
        app_data_hash: 5,
    };
    let mut bytes = [0; 32 * (1 + RESOURCE_FIELDS)];
    for i in 0..1 + RESOURCE_FIELDS {
        bytes[32 * i + 31] = i as u8;
    }
    assert_eq(extend(0, [resource]), low_253_bits(sha256::digest(bytes)));
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
    // The digest that the contract computes for an action of a consumed and a created resource without blobs.
    let consumed = ResourceInstance {
        is_consumed: true,
        tag: 1,
        logic_ref: 2,
        commitment_tree_root: 3,
        app_data_hash: EMPTY_APP_DATA_HASH,
    };
    let created = ResourceInstance {
        is_consumed: false,
        tag: 4,
        logic_ref: 5,
        commitment_tree_root: 0,
        app_data_hash: EMPTY_APP_DATA_HASH,
    };
    assert_eq(
        extend(0, [consumed, created]),
        0x0b60bead17f79a743fd35dc5f1d9948be77dceede95f75c6a7de00414a41138d,
    );
}
