use crate::hash::{self, hash};

/// The root of the action tree of a compliance unit: the hash of the tag of its consumed resource and the tag of its created resource.
pub fn action_tree_root(consumed_tag: Field, created_tag: Field) -> Field {
    hash(hash::ACTION_TREE, [consumed_tag, created_tag])
}

#[test]
fn action_tree_root_depends_on_the_order_of_the_tags() {
    assert(action_tree_root(1, 2) != action_tree_root(2, 1));
}
