use anoma_arm::Fr;
use anoma_arm::field::to_hex;
use anoma_token::public_key_hash;
use p256::ecdsa::SigningKey;

#[test]
fn public_key_hash_matches_noir() -> anyhow::Result<()> {
    let mut secret = [0; 32];
    secret[30..].copy_from_slice(&1001u16.to_be_bytes());
    let key = SigningKey::from_slice(&secret)?;

    let hash: Fr = public_key_hash(key.verifying_key());

    assert_eq!(
        to_hex(hash),
        "0x16c77995e1a58f0bece092eba36efa37d5639b78b99c28828f29ba4e0b09ef00"
    );
    Ok(())
}
