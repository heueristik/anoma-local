use arm::{action::action_tree_root, resource::Resource};
use poseidon::poseidon2::Poseidon2;

/// What the owner of an intent wants for it: at least `min_quantity` of one kind, owned by one nullifier key.
pub struct Terms {
    pub want_logic_ref: Field,
    pub want_label_ref: Field,
    pub min_quantity: u128,
    pub receiver_nullifier_key_commitment: Field,
}

impl Terms {
    pub fn hash(self) -> Field {
        Poseidon2::hash(
            [
                self.want_logic_ref,
                self.want_label_ref,
                self.min_quantity as Field,
                self.receiver_nullifier_key_commitment,
            ],
            4,
        )
    }
}

/// The logic of an intent resource, whose label is the hash of its terms. Anyone can create one. The action that
/// consumes one must create a resource that fills its terms.
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
    assert(created.logic_ref == terms.want_logic_ref, "the created resource has another logic");
    assert(created.label_ref == terms.want_label_ref, "the created resource has another label");
    assert(created.quantity >= terms.min_quantity, "the created quantity is below the minimum");
    assert(
        created.nullifier_key_commitment == terms.receiver_nullifier_key_commitment,
        "the created resource has another owner",
    );
}

#[test]
fn check_consumed_accepts_a_created_resource_that_fills_the_terms() {
    let (intent, terms, created) = filled();
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

#[test(should_fail_with = "the created quantity is below the minimum")]
fn check_consumed_fails_on_a_quantity_below_the_minimum() {
    let (intent, terms, mut created) = filled();
    created.quantity = terms.min_quantity - 1;
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

#[test(should_fail_with = "the label of the intent is not the hash of its terms")]
fn check_consumed_fails_on_terms_that_are_not_the_label() {
    let (intent, mut terms, mut created) = filled();
    terms.min_quantity -= 1;
    created.quantity = terms.min_quantity;
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

fn filled() -> (Resource, Terms, Resource) {
    let terms = Terms {
        want_logic_ref: 7,
        want_label_ref: 8,
        min_quantity: 4,
        receiver_nullifier_key_commitment: 9,
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
        value_ref: 0,
        quantity: terms.min_quantity,
        nonce: 4,
        nullifier_key_commitment: terms.receiver_nullifier_key_commitment,
        is_ephemeral: false,
        rand_seed: 5,
    };
    (intent, terms, created)
}
