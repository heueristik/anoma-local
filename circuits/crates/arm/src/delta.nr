use std::embedded_curve_ops::{
    EmbeddedCurvePoint, EmbeddedCurveScalar, fixed_base_scalar_mul, multi_scalar_mul,
};

/// The delta of a compliance unit: the consumed quantity times its kind base, minus the created quantity times its kind base, plus `rcv` times the generator.
pub fn unit_delta(
    consumed_kind: EmbeddedCurvePoint,
    consumed_quantity: u128,
    created_kind: EmbeddedCurvePoint,
    created_quantity: u128,
    rcv: EmbeddedCurveScalar,
) -> EmbeddedCurvePoint {
    multi_scalar_mul(
        [consumed_kind, -created_kind, EmbeddedCurvePoint::generator()],
        [
            EmbeddedCurveScalar::from_field(consumed_quantity as Field),
            EmbeddedCurveScalar::from_field(created_quantity as Field),
            rcv,
        ],
    )
}

#[test]
fn unit_delta_is_rcv_times_the_generator_if_the_unit_balances() {
    let kind = EmbeddedCurvePoint::generator().double();
    let rcv = EmbeddedCurveScalar::from_field(99);
    assert_eq(unit_delta(kind, 7, kind, 7, rcv), fixed_base_scalar_mul(rcv));
}

#[test]
fn unit_delta_adds_the_net_quantity_times_the_kind_base() {
    let kind = EmbeddedCurvePoint::generator().double();
    let rcv = EmbeddedCurveScalar::from_field(99);
    let net = multi_scalar_mul([kind], [EmbeddedCurveScalar::from_field(3)]);
    assert_eq(unit_delta(kind, 10, kind, 7, rcv), net + fixed_base_scalar_mul(rcv));
}
