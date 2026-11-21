use crate::hash::{self, hash};
use std::embedded_curve_ops::EmbeddedCurvePoint;

/// The largest y coordinate of a kind base, (p - 1) / 2: of the two points with one x, only the one with the smaller y counts.
global MAX_Y: Field = 10944121435919637611123202872628637544274182200208017171849102093287904247808;

/// The point of a kind: a counter whose hash is the x coordinate of a Grumpkin point, and the smaller y of that point.
pub struct KindWitness {
    pub counter: Field,
    pub y: Field,
}

/// Maps a kind to a Grumpkin point with no known discrete logarithm; any counter gives an independent point, so the counter needs no check.
pub fn kind_base(logic_ref: Field, label_ref: Field, witness: KindWitness) -> EmbeddedCurvePoint {
    let x = hash(hash::KIND, [logic_ref, label_ref, witness.counter]);
    assert(witness.y * witness.y == x * x * x - 17, "the kind base is not on the curve");
    assert(!MAX_Y.lt(witness.y), "the kind base must take the smaller y");
    EmbeddedCurvePoint::new(x, witness.y)
}

#[test(should_fail_with = "the kind base is not on the curve")]
fn kind_base_fails_off_the_curve() {
    let _ = kind_base(1, 2, KindWitness { counter: 0, y: 3 });
}

#[test]
fn kind_base_accepts_the_witness_of_the_rust_library() {
    let witness = KindWitness {
        counter: 1,
        y: 0x167c770fd1dbca61ea5ad08f542449a9fe4c520aa90b12358b879051e5b8ac3e,
    };
    let _ = kind_base(1, 2, witness);
}

#[test(should_fail_with = "the kind base must take the smaller y")]
fn kind_base_fails_on_the_larger_y() {
    let y = 0x167c770fd1dbca61ea5ad08f542449a9fe4c520aa90b12358b879051e5b8ac3e;
    let _ = kind_base(1, 2, KindWitness { counter: 1, y: -y });
}
