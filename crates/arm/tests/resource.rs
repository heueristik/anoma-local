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
fn from_fields_gives_the_resource_that_to_fields_encoded() {
    assert_eq!(
        Resource::from_fields(&resource().to_fields()),
        Ok(resource())
    );
}

#[test]
fn from_fields_errors_on_a_quantity_of_more_than_128_bits() {
    let mut fields = resource().to_fields();
    // The quantity is the fourth field.
    fields[3] = Fr::from(u128::MAX) + Fr::from(1u64);

    assert_eq!(
        Resource::from_fields(&fields),
        Err(Error::InvalidField("quantity"))
    );
}

#[test]
fn from_fields_errors_on_an_ephemeral_flag_other_than_zero_or_one() {
    let mut fields = resource().to_fields();
    // The ephemeral flag is the seventh field.
    fields[6] = Fr::from(2u64);

    assert_eq!(
        Resource::from_fields(&fields),
        Err(Error::InvalidField("is_ephemeral"))
    );
}
