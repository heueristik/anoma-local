use anoma_arm::Error;
use anoma_arm::Fr;
use anoma_arm::resource::{NullifierKey, Resource};

fn resource() -> Resource {
    Resource {
        logic_ref: Fr::from(1u64),
        label_ref: Fr::from(2u64),
        value_ref: Fr::from(3u64),
        quantity: u128::MAX,
        nonce: Fr::from(5u64),
        nullifier_key_commitment: NullifierKey(Fr::from(6u64)).commitment(),
        is_ephemeral: true,
        rand_seed: -Fr::from(1u64),
    }
}

#[test]
fn from_bytes_gives_the_resource_that_to_bytes_encoded() {
    assert_eq!(Resource::from_bytes(&resource().to_bytes()), Ok(resource()));
}

#[test]
fn from_bytes_errors_on_a_quantity_of_more_than_128_bits() {
    let mut bytes = resource().to_bytes();
    // The quantity is the fourth word.
    bytes[3 * 32 + 15] = 1;

    assert_eq!(
        Resource::from_bytes(&bytes),
        Err(Error::InvalidWord("quantity"))
    );
}

#[test]
fn from_bytes_errors_on_an_ephemeral_flag_other_than_zero_or_one() {
    let mut bytes = resource().to_bytes();
    // The ephemeral flag is the seventh word.
    bytes[6 * 32 + 31] = 2;

    assert_eq!(
        Resource::from_bytes(&bytes),
        Err(Error::InvalidWord("is_ephemeral"))
    );
}
