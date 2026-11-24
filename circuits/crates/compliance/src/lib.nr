use arm::{
    action::MAX_RESOURCES,
    delta::unit_delta,
    kind::{kind_base, KindWitness},
    mmb::MembershipPath,
    resource::{created_nonce, extend_nullifiers, Resource},
};
use std::embedded_curve_ops::{EmbeddedCurvePoint, EmbeddedCurveScalar};

/// The public data of one resource of a compliance unit. A created resource has the commitment tree root zero.
pub struct ResourceInstance {
    pub is_consumed: bool,
    pub tag: Field,
    pub logic_ref: Field,
    pub commitment_tree_root: Field,
}

/// The private data of one resource. Only a consumed resource uses its nullifier key, and only a persistent consumed
/// resource uses its path.
pub struct ResourceWitness {
    pub resource: Resource,
    pub kind: KindWitness,
    pub nullifier_key: Field,
    pub path: MembershipPath,
}

/// The public outputs of a compliance unit of `K` resources: each resource in the order of the list, and the delta.
pub struct ComplianceInstance<let K: u32> {
    pub resources: [ResourceInstance; K],
    pub delta_x: Field,
    pub delta_y: Field,
}

pub struct ComplianceWitness<let K: u32> {
    pub resources: [ResourceWitness; K],
    pub rcv: EmbeddedCurveScalar,
}

/// Checks a compliance unit of `K` resources, of which at least one is consumed. The nonce of each created resource
/// comes from its index among the created resources and from all consumed nullifiers, in the order of the list.
pub fn check<let K: u32>(instance: ComplianceInstance<K>, witness: ComplianceWitness<K>) {
    assert(
        K <= MAX_RESOURCES,
        "a compliance unit has more resources than an action tree has leaves",
    );
    let mut nullifiers = 0;
    let mut consumed_count = 0;
    let mut kind_bases = [EmbeddedCurvePoint::generator(); K];
    let mut quantities = [0; K];
    let mut is_consumed = [false; K];
    for i in 0..K {
        let public = instance.resources[i];
        let private = witness.resources[i];
        let resource = private.resource;
        assert(
            resource.logic_ref == public.logic_ref,
            "the logic ref is not the logic ref of the resource",
        );
        if public.is_consumed {
            assert(
                resource.nullifier(private.nullifier_key) == public.tag,
                "the tag is not the nullifier of the consumed resource",
            );
            if !resource.is_ephemeral {
                assert(
                    private.path.root(resource.commitment()) == public.commitment_tree_root,
                    "the consumed resource is not in the commitment tree",
                );
            }
            nullifiers = extend_nullifiers(nullifiers, public.tag);
            consumed_count += 1;
        } else {
            // The contract hashes the root of each resource, so a created resource must have one value only.
            assert(
                public.commitment_tree_root == 0,
                "the created resource has a commitment tree root",
            );
        }
        kind_bases[i] = kind_base(resource.logic_ref, resource.label_ref, private.kind);
        quantities[i] = resource.quantity;
        is_consumed[i] = public.is_consumed;
    }
    assert(consumed_count != 0, "the compliance unit consumes no resource");

    let mut created_index = 0;
    for i in 0..K {
        let public = instance.resources[i];
        let resource = witness.resources[i].resource;
        if !public.is_consumed {
            assert(
                resource.nonce == created_nonce(created_index, nullifiers),
                "the nonce of the created resource does not come from the consumed nullifiers",
            );
            assert(
                resource.commitment() == public.tag,
                "the tag is not the commitment of the created resource",
            );
            created_index += 1;
        }
    }

    let delta = unit_delta(kind_bases, quantities, is_consumed, witness.rcv);
    assert(
        (delta.x == instance.delta_x) & (delta.y == instance.delta_y),
        "the delta is not the delta of the compliance unit",
    );
}

#[test]
fn check_accepts_a_unit_of_one_consumed_and_two_created_resources() {
    let (instance, witness) = unit();
    check(instance, witness);
}

#[test]
fn check_accepts_the_created_resources_before_the_consumed_resource() {
    let (instance, witness) = unit();
    let mut swapped_instance = instance;
    let mut swapped_witness = witness;
    swapped_instance.resources =
        [instance.resources[1], instance.resources[2], instance.resources[0]];
    swapped_witness.resources = [witness.resources[1], witness.resources[2], witness.resources[0]];
    check(swapped_instance, swapped_witness);
}

#[test(should_fail_with = "the nonce of the created resource does not come from the consumed nullifiers")]
fn check_fails_on_created_resources_in_another_order() {
    let (instance, witness) = unit();
    let mut swapped_instance = instance;
    let mut swapped_witness = witness;
    swapped_instance.resources =
        [instance.resources[0], instance.resources[2], instance.resources[1]];
    swapped_witness.resources = [witness.resources[0], witness.resources[2], witness.resources[1]];
    check(swapped_instance, swapped_witness);
}

#[test(should_fail_with = "the compliance unit consumes no resource")]
fn check_fails_on_a_unit_without_a_consumed_resource() {
    let (instance, witness) = unit();
    let mut created_instance = instance;
    created_instance.resources[0].is_consumed = false;
    created_instance.resources[0].commitment_tree_root = 0;
    created_instance.resources[0].tag = witness.resources[0].resource.commitment();
    check(created_instance, witness);
}

#[test(should_fail_with = "the created resource has a commitment tree root")]
fn check_fails_on_a_created_resource_with_a_commitment_tree_root() {
    let (mut instance, witness) = unit();
    instance.resources[1].commitment_tree_root = ROOT;
    check(instance, witness);
}

#[test(should_fail_with = "the delta is not the delta of the compliance unit")]
fn check_fails_on_another_created_quantity() {
    let (instance, mut witness) = unit();
    witness.resources[2].resource.quantity += 1;
    let mut changed_instance = instance;
    changed_instance.resources[2].tag = witness.resources[2].resource.commitment();
    check(changed_instance, witness);
}

global NULLIFIER_KEY: Field = 42;
global ROOT: Field = 7;

/// A balanced unit of an ephemeral resource of quantity 5 and two created resources of quantities 2 and 3.
fn unit() -> (ComplianceInstance<3>, ComplianceWitness<3>) {
    // The kind witness of the logic ref 1 and the label ref 2, from the Rust library.
    let kind = KindWitness {
        counter: 1,
        y: 0x167c770fd1dbca61ea5ad08f542449a9fe4c520aa90b12358b879051e5b8ac3e,
    };
    let consumed = Resource {
        logic_ref: 1,
        label_ref: 2,
        value_ref: 3,
        quantity: 5,
        nonce: 4,
        nullifier_key_commitment: arm::resource::nullifier_key_commitment(NULLIFIER_KEY),
        is_ephemeral: true,
        rand_seed: 6,
    };
    let nullifier = consumed.nullifier(NULLIFIER_KEY);
    let nullifiers = extend_nullifiers(0, nullifier);
    let mut first = consumed;
    first.quantity = 2;
    first.is_ephemeral = false;
    first.nonce = created_nonce(0, nullifiers);
    let mut second = first;
    second.quantity = 3;
    second.nonce = created_nonce(1, nullifiers);

    let rcv = EmbeddedCurveScalar::from_field(99);
    let delta = unit_delta(
        [kind_base(1, 2, kind); 3],
        [5, 2, 3],
        [true, false, false],
        rcv,
    );
    let path = MembershipPath {
        siblings: [0; arm::mmb::MAX_PATH_LENGTH],
        sibling_is_left: [false; arm::mmb::MAX_PATH_LENGTH],
        length: 0,
    };
    let instance = ComplianceInstance {
        resources: [
            ResourceInstance {
                is_consumed: true,
                tag: nullifier,
                logic_ref: 1,
                commitment_tree_root: ROOT,
            },
            ResourceInstance {
                is_consumed: false,
                tag: first.commitment(),
                logic_ref: 1,
                commitment_tree_root: 0,
            },
            ResourceInstance {
                is_consumed: false,
                tag: second.commitment(),
                logic_ref: 1,
                commitment_tree_root: 0,
            },
        ],
        delta_x: delta.x,
        delta_y: delta.y,
    };
    let witness = ComplianceWitness {
        resources: [
            ResourceWitness { resource: consumed, kind, nullifier_key: NULLIFIER_KEY, path },
            ResourceWitness { resource: first, kind, nullifier_key: 0, path },
            ResourceWitness { resource: second, kind, nullifier_key: 0, path },
        ],
        rcv,
    };
    (instance, witness)
}
