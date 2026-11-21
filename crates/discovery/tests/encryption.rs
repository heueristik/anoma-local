use anoma_discovery::encryption::{Error, SecretKey, decrypt, encrypt};

#[test]
fn decrypt_gives_the_message_to_the_receiver() -> anyhow::Result<()> {
    let receiver = SecretKey::random();
    let message = b"the plaintext of a resource";

    let ciphertext = encrypt(message, &receiver.public_key())?;

    assert_eq!(decrypt(&ciphertext, &receiver)?, message);
    Ok(())
}

#[test]
fn decrypt_errors_on_the_key_of_another_receiver() -> anyhow::Result<()> {
    let ciphertext = encrypt(b"message", &SecretKey::random().public_key())?;

    let result = decrypt(&ciphertext, &SecretKey::random());

    assert!(matches!(result, Err(Error::Decryption)), "got {result:?}");
    Ok(())
}

#[test]
fn decrypt_errors_on_a_changed_ciphertext() -> anyhow::Result<()> {
    let receiver = SecretKey::random();
    let mut ciphertext = encrypt(b"message", &receiver.public_key())?;
    assert!(
        decrypt(&ciphertext, &receiver).is_ok(),
        "the premise is that the unchanged ciphertext decrypts"
    );

    // The first bytes are the bincode length of the AES ciphertext, so the change hits the AES ciphertext.
    ciphertext[8] ^= 0xff;

    assert!(matches!(
        decrypt(&ciphertext, &receiver),
        Err(Error::Decryption)
    ));
    Ok(())
}
