use anoma_discovery::{Discovery, EciesAddress, EciesDiscovery, EciesSecretKeys, ReceiverAddress};

#[test]
fn discover_gives_the_plaintext_to_the_receiver() -> anyhow::Result<()> {
    let keys = EciesSecretKeys::random();

    let app_data = EciesDiscovery.app_data(&keys.address(), b"plaintext")?;

    assert_eq!(
        EciesDiscovery.discover(&app_data, &keys),
        Some(vec![b"plaintext".to_vec()])
    );
    Ok(())
}

#[test]
fn discover_finds_nothing_for_another_receiver() -> anyhow::Result<()> {
    let app_data = EciesDiscovery.app_data(&EciesSecretKeys::random().address(), b"plaintext")?;

    assert_eq!(
        EciesDiscovery.discover(&app_data, &EciesSecretKeys::random()),
        None
    );
    Ok(())
}

#[test]
fn discover_finds_no_plaintext_for_a_receiver_that_has_only_the_discovery_key() -> anyhow::Result<()>
{
    let keys = EciesSecretKeys::random();
    let app_data = EciesDiscovery.app_data(&keys.address(), b"plaintext")?;
    let other = EciesSecretKeys {
        encryption_key: EciesSecretKeys::random().encryption_key,
        ..keys
    };

    assert_eq!(EciesDiscovery.discover(&app_data, &other), Some(Vec::new()));
    Ok(())
}

#[test]
fn from_bytes_gives_the_address_that_to_bytes_encoded() {
    let address = EciesSecretKeys::random().address();

    assert_eq!(EciesAddress::from_bytes(&address.to_bytes()), Some(address));
}

#[test]
fn from_bytes_fails_on_an_extra_byte() {
    let mut bytes = EciesSecretKeys::random().address().to_bytes();
    assert!(
        EciesAddress::from_bytes(&bytes).is_some(),
        "the premise is that the encoding decodes"
    );

    bytes.push(0);

    assert_eq!(EciesAddress::from_bytes(&bytes), None);
}
