mod public_key;

use arm::resource::Resource;
use public_key::PublicKey;

/// The logic of a token, whose label ref is the key hash of its issuer and whose value ref is the key hash of its
/// owner. The consumption of a token of a quantity needs a P-256 signature over the action tree root: of the owner for
/// a persistent token, and of the issuer for an ephemeral token, whose consumption mints the quantity.
fn main(
    tag: pub Field,
    is_consumed: pub bool,
    action_tree_root: pub Field,
    app_data_hash: pub Field,
    token: Resource,
    nullifier_key: Field,
    signer: PublicKey,
    signature: [u8; 64],
) {
    let _ = app_data_hash;
    if is_consumed {
        check_consumed(
            tag,
            action_tree_root,
            token,
            nullifier_key,
            signer,
            signature,
        );
    }
}

fn check_consumed(
    tag: Field,
    action_tree_root: Field,
    token: Resource,
    nullifier_key: Field,
    signer: PublicKey,
    signature: [u8; 64],
) {
    assert(token.nullifier(nullifier_key) == tag, "the tag is not the nullifier of the token");
    // A token of quantity 0 carries no value, so anyone can consume an ephemeral one to create a surplus.
    if token.quantity != 0 {
        if token.is_ephemeral {
            assert(
                signer.hash() == token.label_ref,
                "the signer is not the issuer of the ephemeral token",
            );
        } else {
            assert(signer.hash() == token.value_ref, "the signer is not the owner of the token");
        }
        assert(
            signer.verify(signature, action_tree_root),
            "the signature is not a signature of the signer over the action tree root",
        );
    }
}

#[test]
fn check_consumed_accepts_a_signature_of_the_owner() {
    check(owned(), OWNER, OWNER_SIGNATURE, ACTION_TREE_ROOT);
}

#[test]
fn check_consumed_accepts_a_signature_of_the_issuer_on_an_ephemeral_token() {
    check(ephemeral(), ISSUER, ISSUER_SIGNATURE, ACTION_TREE_ROOT);
}

#[test]
fn check_consumed_accepts_a_token_of_quantity_0_without_a_signature() {
    let mut token = ephemeral();
    token.quantity = 0;
    check(
        token,
        PublicKey { x: [0; 32], y: [0; 32] },
        [0; 64],
        ACTION_TREE_ROOT,
    );
}

#[test(should_fail_with = "the signer is not the owner of the token")]
fn check_consumed_fails_on_a_persistent_token_that_its_issuer_signs() {
    check(owned(), ISSUER, ISSUER_SIGNATURE, ACTION_TREE_ROOT);
}

#[test(should_fail_with = "the signer is not the issuer of the ephemeral token")]
fn check_consumed_fails_on_an_ephemeral_token_that_its_owner_signs() {
    check(ephemeral(), OWNER, OWNER_SIGNATURE, ACTION_TREE_ROOT);
}

#[test(should_fail_with = "the signature is not a signature of the signer over the action tree root")]
fn check_consumed_fails_on_a_signature_over_another_action_tree_root() {
    check(owned(), OWNER, OWNER_SIGNATURE, ACTION_TREE_ROOT + 1);
}

#[test(should_fail_with = "the tag is not the nullifier of the token")]
fn check_consumed_fails_on_a_tag_of_another_token() {
    let token = owned();
    let mut other = token;
    other.nonce += 1;
    check_consumed(
        other.nullifier(NULLIFIER_KEY),
        ACTION_TREE_ROOT,
        token,
        NULLIFIER_KEY,
        OWNER,
        OWNER_SIGNATURE,
    );
}

#[test]
fn public_key_hash_matches_the_rust_crate() {
    assert_eq(OWNER.hash(), 0x16c77995e1a58f0bece092eba36efa37d5639b78b99c28828f29ba4e0b09ef00);
}

global NULLIFIER_KEY: Field = 42;

/// The root that the signatures of the test keys sign.
global ACTION_TREE_ROOT: Field = 77;

/// Checks the consumption of `token` with its own nullifier as the tag.
fn check(token: Resource, signer: PublicKey, signature: [u8; 64], action_tree_root: Field) {
    check_consumed(
        token.nullifier(NULLIFIER_KEY),
        action_tree_root,
        token,
        NULLIFIER_KEY,
        signer,
        signature,
    );
}

fn owned() -> Resource {
    Resource {
        logic_ref: 1,
        label_ref: ISSUER.hash(),
        value_ref: OWNER.hash(),
        quantity: 5,
        nonce: 2,
        nullifier_key_commitment: arm::resource::nullifier_key_commitment(NULLIFIER_KEY),
        is_ephemeral: false,
        rand_seed: 3,
    }
}

fn ephemeral() -> Resource {
    let mut token = owned();
    token.is_ephemeral = true;
    token
}

// The keys of the secret scalars 1001 and 1002, and their signatures over the 32 big-endian bytes of `ACTION_TREE_ROOT`.
global OWNER: PublicKey = PublicKey {
    x: [
        0xcc, 0xf7, 0xa8, 0x7b, 0xe5, 0xca, 0x16, 0xea, 0xc0, 0x08, 0x92, 0x3d, 0xab, 0x1e, 0x28,
        0xb8, 0x12, 0x31, 0x05, 0xaa, 0x3c, 0xcd, 0x99, 0x18, 0x70, 0x52, 0x22, 0xca, 0x5d, 0x3a,
        0x7c, 0xc6,
    ],
    y: [
        0xbb, 0x27, 0x2e, 0xa8, 0xe5, 0x53, 0x08, 0xf9, 0x02, 0x52, 0xc9, 0x6a, 0x2e, 0x95, 0x6b,
        0xe8, 0xb6, 0x36, 0xe5, 0x78, 0xe3, 0x2d, 0x36, 0x63, 0xff, 0x5b, 0x6f, 0xfe, 0x5e, 0x9d,
        0x6c, 0xb2,
    ],
};
global OWNER_SIGNATURE: [u8; 64] = [
    0x39, 0xb1, 0xda, 0xd1, 0x9b, 0x3c, 0x48, 0x7c, 0xfe, 0x47, 0x39, 0xa2, 0xba, 0xf2, 0x63, 0xa8,
    0x7f, 0x45, 0x99, 0x32, 0xbf, 0x0c, 0x4f, 0xfa, 0x28, 0xaf, 0x56, 0xc4, 0xd9, 0xc2, 0x45, 0xdf,
    0x17, 0xb3, 0xfd, 0x68, 0x7f, 0xf7, 0xc2, 0x52, 0x41, 0x10, 0xc5, 0x3b, 0x37, 0x1e, 0x6e, 0x6d,
    0xf5, 0x00, 0x9f, 0xbb, 0xbc, 0x90, 0x2c, 0x3d, 0xf6, 0x3c, 0xff, 0xe6, 0x27, 0xd5, 0x4a, 0x1e,
];
global ISSUER: PublicKey = PublicKey {
    x: [
        0xf7, 0x28, 0xc5, 0x7e, 0x47, 0x11, 0x5f, 0x2e, 0x20, 0xcc, 0x0d, 0xcb, 0xf9, 0x68, 0x0a,
        0x1a, 0x9a, 0xc2, 0x93, 0x09, 0xfc, 0x28, 0x92, 0x8a, 0xf7, 0xea, 0x01, 0xcd, 0x36, 0xb4,
        0xb7, 0xb9,
    ],
    y: [
        0x14, 0xe3, 0x78, 0x48, 0xc3, 0xa6, 0xae, 0xba, 0x90, 0x85, 0xe3, 0x14, 0x26, 0x20, 0x67,
        0x1d, 0xa9, 0xa2, 0x46, 0xe4, 0x1b, 0xf6, 0xef, 0x3d, 0xa6, 0xa4, 0xc6, 0x12, 0xe6, 0xf4,
        0x9c, 0x29,
    ],
};
global ISSUER_SIGNATURE: [u8; 64] = [
    0xee, 0x6a, 0x10, 0xfd, 0x29, 0x3f, 0xa1, 0xb9, 0x06, 0x5a, 0x67, 0x5e, 0x31, 0xe4, 0xea, 0x21,
    0x85, 0xfd, 0x57, 0x41, 0x62, 0x9e, 0x10, 0xc1, 0xca, 0x14, 0x87, 0x29, 0xfc, 0x33, 0x91, 0x87,
    0x42, 0xe1, 0x6c, 0x74, 0xc4, 0x89, 0x44, 0x1b, 0x5d, 0x12, 0xec, 0x1f, 0xf8, 0x28, 0x0c, 0x19,
    0x03, 0x4a, 0xa6, 0xa2, 0x0f, 0x6d, 0xa3, 0x83, 0x3b, 0xb3, 0xbc, 0xd0, 0x4a, 0x46, 0x79, 0x60,
];
