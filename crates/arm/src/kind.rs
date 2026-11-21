//! Kind bases: Grumpkin points with no known discrete logarithm, one for each logic ref and label ref.

use ark_ff::{Field, PrimeField};

use crate::hash::{Domain, hash};
use crate::{Fr, Point};

/// The point of a kind, and the witness that the compliance circuit takes for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kind {
    pub base: Point,
    pub counter: Fr,
    pub y: Fr,
}

impl Kind {
    /// Takes the first counter whose hash is the x coordinate of a Grumpkin point, and of its two points the one
    /// with the smaller y.
    pub fn new(logic_ref: Fr, label_ref: Fr) -> Self {
        let mut counter = Fr::from(0u64);
        loop {
            let x = hash(Domain::Kind, &[logic_ref, label_ref, counter]);
            if let Some(root) = (x * x * x - Fr::from(17u64)).sqrt() {
                let y = if is_small(root) { root } else { -root };
                return Self {
                    base: Point::new_unchecked(x, y),
                    counter,
                    y,
                };
            }
            counter += Fr::from(1u64);
        }
    }
}

/// Whether a value is at most (p - 1) / 2.
fn is_small(value: Fr) -> bool {
    value.into_bigint() <= Fr::MODULUS_MINUS_ONE_DIV_TWO
}
