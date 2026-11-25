//! The delta of a compliance unit: a Grumpkin point that sums to the generator times the sum of the rcv values exactly
//! when the quantities of each kind balance.

use ark_ec::{AffineRepr, CurveGroup};

use crate::{Point, Scalar};

/// The share of one resource in the delta of its compliance unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceDelta {
    pub kind_base: Point,
    pub quantity: u128,
    pub is_consumed: bool,
}

/// Each consumed quantity times its kind base, minus each created quantity times its kind base, plus `rcv` times the
/// generator.
pub fn unit_delta(resources: &[ResourceDelta], rcv: Scalar) -> Point {
    resources
        .iter()
        .fold(Point::generator() * rcv, |sum, resource| {
            let share = resource.kind_base * Scalar::from(resource.quantity);
            if resource.is_consumed {
                sum + share
            } else {
                sum - share
            }
        })
        .into_affine()
}
