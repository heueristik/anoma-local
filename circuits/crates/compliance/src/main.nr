use arm::{
    delta::unit_delta,
    kind::{kind_base, KindWitness},
    mmb::MembershipPath,
    resource::{created_nonce, Resource},
};
use std::embedded_curve_ops::EmbeddedCurveScalar;

/// A compliance unit of one consumed and one created resource.
fn main(
    nullifier: pub Field,
    consumed_logic_ref: pub Field,
    commitment_tree_root: pub Field,
    commitment: pub Field,
    created_logic_ref: pub Field,
    delta_x: pub Field,
    delta_y: pub Field,
    consumed: Resource,
    nullifier_key: Field,
    path: MembershipPath,
    consumed_kind: KindWitness,
    created: Resource,
    created_kind: KindWitness,
    rcv: EmbeddedCurveScalar,
) {
    assert(
        consumed.nullifier(nullifier_key) == nullifier,
        "the nullifier is not the nullifier of the consumed resource",
    );
    assert(
        consumed.logic_ref == consumed_logic_ref,
        "the logic ref is not the logic ref of the consumed resource",
    );
    if !consumed.is_ephemeral {
        assert(
            path.root(consumed.commitment()) == commitment_tree_root,
            "the consumed resource is not in the commitment tree",
        );
    }

    assert(
        created.nonce == created_nonce(nullifier),
        "the nonce of the created resource does not come from the nullifier",
    );
    assert(
        created.commitment() == commitment,
        "the commitment is not the commitment of the created resource",
    );
    assert(
        created.logic_ref == created_logic_ref,
        "the logic ref is not the logic ref of the created resource",
    );

    let delta = unit_delta(
        kind_base(consumed.logic_ref, consumed.label_ref, consumed_kind),
        consumed.quantity,
        kind_base(created.logic_ref, created.label_ref, created_kind),
        created.quantity,
        rcv,
    );
    assert(
        (delta.x == delta_x) & (delta.y == delta_y),
        "the delta is not the delta of the compliance unit",
    );
}
