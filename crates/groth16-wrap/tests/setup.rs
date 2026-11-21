//! Checks that an error of the Go library reaches the caller through the C interface. It runs by default, because
//! the setup fails before it compiles the circuit.

mod common;

use anoma_groth16_wrap::{Error, FIELD_BYTES, single_party_setup};

#[test]
fn single_party_setup_fails_on_a_verifying_key_without_its_last_field() -> anyhow::Result<()> {
    let verifying_key = common::fixture("vk")?;
    let fields = verifying_key.len() / FIELD_BYTES;
    let directory = common::keys_directory("keys-of-a-short-verifying-key");
    assert!(
        !directory.exists(),
        "the premise is that {} does not exist",
        directory.display()
    );

    let result = single_party_setup(
        &verifying_key[..verifying_key.len() - FIELD_BYTES],
        &directory,
    );

    let Err(Error::Failed(message)) = result else {
        anyhow::bail!("expected a failure, got {result:?}");
    };
    let expected = format!("vk has {} fields, expected {fields}", fields - 1);
    assert!(
        message.contains(&expected),
        "the message {message:?} does not contain {expected:?}"
    );
    assert!(
        !directory.exists(),
        "the setup wrote {} before it failed",
        directory.display()
    );
    Ok(())
}
