use arm::{action::action_tree_root, resource::Resource};
use poseidon::poseidon2::Poseidon2;

/// What the owner of an intent wants for it: each field of the resource that fills it, except the nonce, which comes from the nullifier of the intent.
pub struct Terms {
    pub want_logic_ref: Field,
    pub want_label_ref: Field,
    pub want_value_ref: Field,
    pub min_quantity: u128,
    pub receiver_nullifier_key_commitment: Field,
    pub want_rand_seed: Field,
}

impl Terms {
    pub fn hash(self) -> Field {
        Poseidon2::hash(
            [
                self.want_logic_ref,
                self.want_label_ref,
                self.want_value_ref,
                self.min_quantity as Field,
                self.receiver_nullifier_key_commitment,
                self.want_rand_seed,
            ],
            6,
        )
    }
}

/// The logic of an intent resource, whose label is the hash of its terms. Anyone can create one. The action that
/// consumes one must create the resource of its terms.
fn main(
    tag: pub Field,
    is_consumed: pub bool,
    action_tree_root: pub Field,
    app_data_hash: pub Field,
    intent: Resource,
    nullifier_key: Field,
    terms: Terms,
    created: Resource,
) {
    let _ = app_data_hash;
    if is_consumed {
        check_consumed(tag, action_tree_root, intent, nullifier_key, terms, created);
    }
}

fn check_consumed(
    tag: Field,
    action_tree_root_of_action: Field,
    intent: Resource,
    nullifier_key: Field,
    terms: Terms,
    created: Resource,
) {
    assert(intent.nullifier(nullifier_key) == tag, "the tag is not the nullifier of the intent");
    // The kind of the intent then fixes its terms, so the deltas cancel only if each consumed intent fills them.
    assert(
        intent.label_ref == terms.hash(),
        "the label of the intent is not the hash of its terms",
    );
    // One consumed intent of quantity 1 cancels each created one, so that each fills its terms once.
    assert(intent.quantity == 1, "the quantity of the intent is not 1");
    assert(
        action_tree_root(tag, created.commitment()) == action_tree_root_of_action,
        "the resource is not the created resource of the action",
    );
    // The compliance unit fixes the nonce, so the terms fix the whole created resource, and its receiver can compute it.
    assert(created.logic_ref == terms.want_logic_ref, "the created resource has another logic");
    assert(created.label_ref == terms.want_label_ref, "the created resource has another label");
    assert(created.value_ref == terms.want_value_ref, "the created resource has another value");
    assert(
        created.quantity == terms.min_quantity,
        "the created quantity is not the quantity of the terms",
    );
    assert(
        created.nullifier_key_commitment == terms.receiver_nullifier_key_commitment,
        "the created resource has another owner",
    );
    assert(!created.is_ephemeral, "the created resource is ephemeral");
    assert(
        created.rand_seed == terms.want_rand_seed,
        "the created resource has another random seed",
    );
}

#[test]
fn check_consumed_accepts_the_created_resource_of_the_terms() {
    let (intent, terms, created) = filled();
    check(intent, terms, created);
}

#[test(should_fail_with = "the created quantity is not the quantity of the terms")]
fn check_consumed_fails_on_a_quantity_above_the_terms() {
    let (intent, terms, mut created) = filled();
    created.quantity = terms.min_quantity + 1;
    check(intent, terms, created);
}

#[test(should_fail_with = "the created resource has another value")]
fn check_consumed_fails_on_another_value() {
    let (intent, terms, mut created) = filled();
    created.value_ref += 1;
    check(intent, terms, created);
}

#[test(should_fail_with = "the created resource is ephemeral")]
fn check_consumed_fails_on_an_ephemeral_resource() {
    let (intent, terms, mut created) = filled();
    created.is_ephemeral = true;
    check(intent, terms, created);
}

#[test(should_fail_with = "the created resource has another random seed")]
fn check_consumed_fails_on_another_random_seed() {
    let (intent, terms, mut created) = filled();
    created.rand_seed += 1;
    check(intent, terms, created);
}

#[test(should_fail_with = "the label of the intent is not the hash of its terms")]
fn check_consumed_fails_on_terms_that_are_not_the_label() {
    let (intent, mut terms, mut created) = filled();
    terms.min_quantity -= 1;
    created.quantity = terms.min_quantity;
    check(intent, terms, created);
}

#[test(should_fail_with = "the resource is not the created resource of the action")]
fn check_consumed_fails_on_a_resource_of_another_action() {
    let (intent, terms, created) = filled();
    let tag = intent.nullifier(NULLIFIER_KEY);
    let mut other = created;
    other.nonce += 1;
    check_consumed(
        tag,
        action_tree_root(tag, other.commitment()),
        intent,
        NULLIFIER_KEY,
        terms,
        created,
    );
}

global NULLIFIER_KEY: Field = 42;

/// Checks `created` as the created resource of the action that consumes `intent`.
fn check(intent: Resource, terms: Terms, created: Resource) {
    let tag = intent.nullifier(NULLIFIER_KEY);
    check_consumed(
        tag,
        action_tree_root(tag, created.commitment()),
        intent,
        NULLIFIER_KEY,
        terms,
        created,
    );
}

fn filled() -> (Resource, Terms, Resource) {
    let terms = Terms {
        want_logic_ref: 7,
        want_label_ref: 8,
        want_value_ref: 10,
        min_quantity: 4,
        receiver_nullifier_key_commitment: 9,
        want_rand_seed: 5,
    };
    let intent = Resource {
        logic_ref: 1,
        label_ref: terms.hash(),
        value_ref: 0,
        quantity: 1,
        nonce: 2,
        nullifier_key_commitment: arm::resource::nullifier_key_commitment(NULLIFIER_KEY),
        is_ephemeral: true,
        rand_seed: 3,
    };
    let created = Resource {
        logic_ref: terms.want_logic_ref,
        label_ref: terms.want_label_ref,
        value_ref: terms.want_value_ref,
        quantity: terms.min_quantity,
        nonce: 4,
        nullifier_key_commitment: terms.receiver_nullifier_key_commitment,
        is_ephemeral: false,
        rand_seed: terms.want_rand_seed,
    };
    (intent, terms, created)
}
