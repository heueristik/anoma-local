//! Conversions of field elements to the forms that the contract and the Noir inputs use.

use ark_ec::AffineRepr;
use ark_ff::{BigInteger, PrimeField};

use crate::{Error, Fr, Point, Scalar};

/// The 32 big-endian bytes of an element of a BN254 or Grumpkin field, as the contract stores a tag.
pub fn to_bytes<F: PrimeField>(value: F) -> [u8; 32] {
    let mut bytes = [0; 32];
    bytes.copy_from_slice(&value.into_bigint().to_bytes_be());
    bytes
}

/// The field element of 32 big-endian bytes, if they are smaller than the modulus.
pub fn from_bytes<F: PrimeField>(bytes: [u8; 32]) -> Result<F, Error> {
    let value = F::from_be_bytes_mod_order(&bytes);
    if to_bytes(value) == bytes {
        Ok(value)
    } else {
        Err(Error::NotInField(format!("0x{}", hex::encode(bytes))))
    }
}

/// The hexadecimal form with a `0x` prefix and 64 digits, as nargo reads a field.
pub fn to_hex(value: Fr) -> String {
    format!("0x{}", hex::encode(to_bytes(value)))
}

/// The low and high 128 bits of a Grumpkin scalar, as Noir's `EmbeddedCurveScalar` holds them.
pub fn scalar_limbs(scalar: Scalar) -> (Fr, Fr) {
    let bytes = scalar.into_bigint().to_bytes_be();
    (
        Fr::from_be_bytes_mod_order(&bytes[16..]),
        Fr::from_be_bytes_mod_order(&bytes[..16]),
    )
}

/// The coordinates of a Grumpkin point, with the point at infinity as (0, 0), as Noir's `EmbeddedCurvePoint` holds it.
pub fn coordinates(point: Point) -> (Fr, Fr) {
    point.xy().unwrap_or_default()
}
