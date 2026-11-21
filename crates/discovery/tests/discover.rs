use anoma_discovery::{ReceiverKeys, app_data, discover};

#[test]
fn discover_gives_the_plaintext_to_the_receiver() -> anyhow::Result<()> {
    let keys = ReceiverKeys::random();

    let app_data = app_data(&keys.receiver(), b"plaintext")?;

    assert_eq!(
        discover(&app_data, &keys),
        Some(vec![b"plaintext".to_vec()])
    );
    Ok(())
}

#[test]
fn discover_finds_nothing_for_another_receiver() -> anyhow::Result<()> {
    let app_data = app_data(&ReceiverKeys::random().receiver(), b"plaintext")?;

    assert_eq!(discover(&app_data, &ReceiverKeys::random()), None);
    Ok(())
}

#[test]
fn discover_finds_no_plaintext_for_a_receiver_that_has_only_the_discovery_key() -> anyhow::Result<()>
{
    let keys = ReceiverKeys::random();
    let app_data = app_data(&keys.receiver(), b"plaintext")?;
    let other = ReceiverKeys {
        encryption_key: ReceiverKeys::random().encryption_key,
        ..keys
    };

    assert_eq!(discover(&app_data, &other), Some(Vec::new()));
    Ok(())
}
