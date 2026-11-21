use anoma_arm::Fr;
use anoma_discovery::{
    Discovery, GrumpkinAddress, GrumpkinDiscovery, GrumpkinSecretKeys, ReceiverAddress,
};

fn plaintext() -> Vec<Fr> {
    [1u64, 2, 3].map(Fr::from).to_vec()
}

#[test]
fn discover_gives_the_plaintext_to_the_receiver() -> anyhow::Result<()> {
    let keys = GrumpkinSecretKeys::random();

    let app_data = GrumpkinDiscovery.app_data(&keys.address(), &plaintext())?;

    assert_eq!(
        GrumpkinDiscovery.discover(&app_data, &keys),
        Some(vec![plaintext()])
    );
    Ok(())
}

#[test]
fn discover_finds_nothing_for_another_receiver() -> anyhow::Result<()> {
    let app_data =
        GrumpkinDiscovery.app_data(&GrumpkinSecretKeys::random().address(), &plaintext())?;

    assert_eq!(
        GrumpkinDiscovery.discover(&app_data, &GrumpkinSecretKeys::random()),
        None
    );
    Ok(())
}

#[test]
fn discover_finds_no_plaintext_for_a_receiver_that_has_only_the_discovery_key() -> anyhow::Result<()>
{
    let keys = GrumpkinSecretKeys::random();
    let app_data = GrumpkinDiscovery.app_data(&keys.address(), &plaintext())?;
    let other = GrumpkinSecretKeys {
        encryption_key: GrumpkinSecretKeys::random().encryption_key,
        ..keys
    };

    assert_eq!(
        GrumpkinDiscovery.discover(&app_data, &other),
        Some(Vec::new())
    );
    Ok(())
}

#[test]
fn from_fields_gives_the_address_that_to_fields_encoded() {
    let address = GrumpkinSecretKeys::random().address();

    assert_eq!(
        GrumpkinAddress::from_fields(&address.to_fields()),
        Some(address)
    );
}

#[test]
fn from_fields_fails_on_an_extra_field() {
    let mut fields = GrumpkinSecretKeys::random().address().to_fields();
    assert!(
        GrumpkinAddress::from_fields(&fields).is_some(),
        "the premise is that the encoding decodes"
    );

    fields.push(Fr::from(0u64));

    assert_eq!(GrumpkinAddress::from_fields(&fields), None);
}
