use anoma_discovery::{Discovery, EciesDiscovery, Receiver, ReceiverAddress, ReceiverKeys};

#[test]
fn discover_gives_the_plaintext_to_the_receiver() -> anyhow::Result<()> {
    let keys = ReceiverKeys::random();

    let app_data = EciesDiscovery.app_data(&keys.receiver(), b"plaintext")?;

    assert_eq!(
        EciesDiscovery.discover(&app_data, &keys),
        Some(vec![b"plaintext".to_vec()])
    );
    Ok(())
}

#[test]
fn discover_finds_nothing_for_another_receiver() -> anyhow::Result<()> {
    let app_data = EciesDiscovery.app_data(&ReceiverKeys::random().receiver(), b"plaintext")?;

    assert_eq!(
        EciesDiscovery.discover(&app_data, &ReceiverKeys::random()),
        None
    );
    Ok(())
}

#[test]
fn discover_finds_no_plaintext_for_a_receiver_that_has_only_the_discovery_key() -> anyhow::Result<()>
{
    let keys = ReceiverKeys::random();
    let app_data = EciesDiscovery.app_data(&keys.receiver(), b"plaintext")?;
    let other = ReceiverKeys {
        encryption_key: ReceiverKeys::random().encryption_key,
        ..keys
    };

    assert_eq!(EciesDiscovery.discover(&app_data, &other), Some(Vec::new()));
    Ok(())
}

#[test]
fn from_bytes_gives_the_receiver_that_to_bytes_encoded() {
    let receiver = ReceiverKeys::random().receiver();

    assert_eq!(Receiver::from_bytes(&receiver.to_bytes()), Some(receiver));
}

#[test]
fn from_bytes_fails_on_an_extra_byte() {
    let mut bytes = ReceiverKeys::random().receiver().to_bytes();
    assert!(
        Receiver::from_bytes(&bytes).is_some(),
        "the premise is that the encoding decodes"
    );

    bytes.push(0);

    assert_eq!(Receiver::from_bytes(&bytes), None);
}
