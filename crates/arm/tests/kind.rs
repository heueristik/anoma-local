use anoma_arm::Fr;
use anoma_arm::field::to_hex;
use anoma_arm::kind::Kind;
use ark_ff::PrimeField;

#[test]
fn new_gives_a_point_on_the_curve_with_the_smaller_y() {
    for label in 0..20u64 {
        let kind = Kind::new(Fr::from(1u64), Fr::from(label));
        assert!(kind.base.is_on_curve(), "label {label}");
        assert!(
            kind.base.y.into_bigint() <= Fr::MODULUS_MINUS_ONE_DIV_TWO,
            "label {label}"
        );
    }
}

#[test]
fn new_matches_the_witness_that_the_noir_library_accepts() {
    // The witness of `kind_base_accepts_the_witness_of_the_rust_library` in the Noir library.
    let kind = Kind::new(Fr::from(1u64), Fr::from(2u64));
    assert_eq!(kind.counter, Fr::from(1u64));
    assert_eq!(
        to_hex(kind.base.y),
        "0x167c770fd1dbca61ea5ad08f542449a9fe4c520aa90b12358b879051e5b8ac3e"
    );
}
