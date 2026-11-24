use std::embedded_curve_ops::{
    EmbeddedCurvePoint, EmbeddedCurveScalar, fixed_base_scalar_mul, multi_scalar_mul,
};

/// The delta of a compliance unit: each consumed quantity times its kind base, minus each created quantity times its
/// kind base, plus `rcv` times the generator.
pub fn unit_delta<let K: u32>(
    kind_bases: [EmbeddedCurvePoint; K],
    quantities: [u128; K],
    is_consumed: [bool; K],
    rcv: EmbeddedCurveScalar,
) -> EmbeddedCurvePoint {
    let mut points = [EmbeddedCurvePoint::generator(); K + 1];
    let mut scalars = [rcv; K + 1];
    for i in 0..K {
        points[i] = if is_consumed[i] {
            kind_bases[i]
        } else {
            -kind_bases[i]
        };
        scalars[i] = EmbeddedCurveScalar::from_field(quantities[i] as Field);
    }
    multi_scalar_mul(points, scalars)
}

#[test]
fn unit_delta_is_rcv_times_the_generator_if_the_unit_balances() {
    let kind = EmbeddedCurvePoint::generator().double();
    let rcv = EmbeddedCurveScalar::from_field(99);
    let delta = unit_delta([kind, kind, kind], [7, 3, 4], [true, false, false], rcv);
    assert_eq(delta, fixed_base_scalar_mul(rcv));
}

#[test]
fn unit_delta_adds_the_net_quantity_times_the_kind_base() {
    let kind = EmbeddedCurvePoint::generator().double();
    let rcv = EmbeddedCurveScalar::from_field(99);
    let net = multi_scalar_mul([kind], [EmbeddedCurveScalar::from_field(3)]);
    let delta = unit_delta([kind, kind], [10, 7], [true, false], rcv);
    assert_eq(delta, net + fixed_base_scalar_mul(rcv));
}
