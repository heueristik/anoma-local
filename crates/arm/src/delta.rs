//! The delta of a compliance unit: a Grumpkin point that sums to the generator times the sum of the rcv values exactly
//! when the quantities of each kind balance.

use ark_ec::{AffineRepr, CurveGroup};

use crate::{Point, Scalar};

/// The consumed quantity times its kind base, minus the created quantity times its kind base, plus `rcv` times the
/// generator.
pub fn unit_delta(
    consumed_kind: Point,
    consumed_quantity: u128,
    created_kind: Point,
    created_quantity: u128,
    rcv: Scalar,
) -> Point {
    (consumed_kind * Scalar::from(consumed_quantity)
        - created_kind * Scalar::from(created_quantity)
        + Point::generator() * rcv)
        .into_affine()
}
