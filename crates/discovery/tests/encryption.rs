use anoma_arm::Fr;
use anoma_discovery::encryption::{Ciphertext, SecretKey, decrypt, encrypt};

fn message() -> Vec<Fr> {
    [1u64, 2, 3].map(Fr::from).to_vec()
}

#[test]
fn decrypt_gives_the_message_to_the_receiver() {
    let receiver = SecretKey::random();

    let ciphertext = encrypt(&message(), &receiver.public_key(), &SecretKey::random());

    assert_eq!(decrypt(&ciphertext, &receiver), Some(message()));
}

#[test]
fn decrypt_fails_for_the_key_of_another_receiver() {
    let ciphertext = encrypt(
        &message(),
        &SecretKey::random().public_key(),
        &SecretKey::random(),
    );

    assert_eq!(decrypt(&ciphertext, &SecretKey::random()), None);
}

#[test]
fn decrypt_fails_on_a_changed_masked_field() {
    let receiver = SecretKey::random();
    let mut ciphertext = encrypt(&message(), &receiver.public_key(), &SecretKey::random());
    assert!(
        decrypt(&ciphertext, &receiver).is_some(),
        "the premise is that the unchanged ciphertext decrypts"
    );

    ciphertext.masked[0] += Fr::from(1u64);

    assert_eq!(decrypt(&ciphertext, &receiver), None);
}

#[test]
fn from_bytes_gives_the_ciphertext_that_to_bytes_encoded() {
    let ciphertext = encrypt(
        &message(),
        &SecretKey::random().public_key(),
        &SecretKey::random(),
    );

    assert_eq!(
        Ciphertext::from_bytes(&ciphertext.to_bytes()),
        Some(ciphertext)
    );
}

#[test]
fn from_bytes_fails_on_an_extra_byte() {
    let ciphertext = encrypt(
        &message(),
        &SecretKey::random().public_key(),
        &SecretKey::random(),
    );
    let mut bytes = ciphertext.to_bytes();
    assert!(
        Ciphertext::from_bytes(&bytes).is_some(),
        "the premise is that the encoding decodes"
    );

    bytes.push(0);

    assert_eq!(Ciphertext::from_bytes(&bytes), None);
}

#[test]
fn from_bytes_fails_on_an_ephemeral_key_off_the_curve() {
    let ciphertext = encrypt(
        &message(),
        &SecretKey::random().public_key(),
        &SecretKey::random(),
    );
    let mut bytes = ciphertext.to_bytes();
    assert!(
        Ciphertext::from_bytes(&bytes).is_some(),
        "the premise is that the encoding decodes"
    );

    // The last byte of the first word is the lowest byte of the x coordinate of the ephemeral key.
    bytes[31] ^= 0xff;

    assert_eq!(Ciphertext::from_bytes(&bytes), None);
}
