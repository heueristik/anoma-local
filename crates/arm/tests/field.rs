use anoma_arm::field::{coordinates, point_from_coordinates, scalar_from_limbs, scalar_limbs};
use anoma_arm::{Fr, Point, Scalar};
use ark_ec::AffineRepr;

#[test]
fn scalar_from_limbs_gives_the_scalar_that_scalar_limbs_split() {
    // The largest scalar fills the high limb as far as the modulus allows.
    let scalar = -Scalar::from(1u64);
    let (low, high) = scalar_limbs(scalar);

    assert_eq!(scalar_from_limbs(low, high), Some(scalar));
}

#[test]
fn scalar_from_limbs_fails_on_a_limb_of_more_than_128_bits() {
    let too_large = Fr::from(u128::MAX) + Fr::from(1u64);

    assert_eq!(scalar_from_limbs(too_large, Fr::from(0u64)), None);
}

#[test]
fn point_from_coordinates_gives_the_point_that_coordinates_split() {
    let (x, y) = coordinates(Point::generator());

    assert_eq!(point_from_coordinates(x, y), Some(Point::generator()));
}

#[test]
fn point_from_coordinates_fails_on_the_point_at_infinity() {
    let (x, y) = coordinates(Point::zero());

    assert_eq!(point_from_coordinates(x, y), None);
}
