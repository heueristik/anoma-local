// Exercises Barretenberg's elliptic (Grumpkin add/double) gates, which the other test circuits never use.
use std::embedded_curve_ops::{EmbeddedCurveScalar, embedded_curve_add, fixed_base_scalar_mul, multi_scalar_mul};

fn main(k: Field, s: Field, t: Field) -> pub Field {
    let p = fixed_base_scalar_mul(EmbeddedCurveScalar::from_field(k));

    let q = fixed_base_scalar_mul(EmbeddedCurveScalar::from_field(t));

    let r = multi_scalar_mul([p, q], [EmbeddedCurveScalar::from_field(s), EmbeddedCurveScalar::from_field(t)]);

    let u = embedded_curve_add(r, p);

    u.x + u.y
}
