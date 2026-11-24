use crate::hash::{self, hash};

/// The largest number of resources of an action: the number of leaves of its action tree.
pub global MAX_RESOURCES: u32 = 8;

/// The depth of an action tree, so that a path has the same length for each size of action.
pub global ACTION_TREE_DEPTH: u32 = 3;

/// The leaves after the last tag of an action; no tag is zero, since each tag is a Poseidon2 hash.
pub global PADDING_LEAF: Field = 0;

/// The root of the action tree over the tags of an action, in the order of its list, with padding leaves after them.
pub fn action_tree_root<let K: u32>(tags: [Field; K]) -> Field {
    action_tree_nodes(tags)[0]
}

/// The path of the tag at `index` in the action tree over `tags`.
pub fn action_tree_path<let K: u32>(tags: [Field; K], index: u32) -> ActionTreePath {
    let nodes = action_tree_nodes(tags);
    let mut siblings = [PADDING_LEAF; ACTION_TREE_DEPTH];
    let mut node = MAX_RESOURCES - 1 + index;
    for level in 0..ACTION_TREE_DEPTH {
        // A left child has an odd index, so its sibling is the next node.
        let sibling = if node % 2 == 1 { node + 1 } else { node - 1 };
        siblings[level] = nodes[sibling];
        node = (node - 1) / 2;
    }
    ActionTreePath { siblings, index }
}

/// The path of a leaf of an action tree: the siblings from the leaf up, and the position of the leaf.
pub struct ActionTreePath {
    pub siblings: [Field; ACTION_TREE_DEPTH],
    pub index: u32,
}

impl ActionTreePath {
    /// Computes the root from the leaf at the position of the path.
    pub fn root(self, leaf: Field) -> Field {
        let is_right: [bool; ACTION_TREE_DEPTH] = (self.index as Field).to_le_bits();
        let mut node = leaf;
        for level in 0..ACTION_TREE_DEPTH {
            let (left, right) = if is_right[level] {
                (self.siblings[level], node)
            } else {
                (node, self.siblings[level])
            };
            node = node_hash(left, right);
        }
        node
    }
}

/// The nodes of the action tree over `tags`. Node i has the children 2i + 1 and 2i + 2, so the leaves are the last
/// MAX_RESOURCES nodes.
fn action_tree_nodes<let K: u32>(tags: [Field; K]) -> [Field; 2 * MAX_RESOURCES - 1] {
    assert(K <= MAX_RESOURCES, "an action has more resources than an action tree has leaves");
    let mut nodes = [PADDING_LEAF; 2 * MAX_RESOURCES - 1];
    for i in 0..K {
        nodes[MAX_RESOURCES - 1 + i] = tags[i];
    }
    for j in 0..MAX_RESOURCES - 1 {
        let i = MAX_RESOURCES - 2 - j;
        nodes[i] = node_hash(nodes[2 * i + 1], nodes[2 * i + 2]);
    }
    nodes
}

fn node_hash(left: Field, right: Field) -> Field {
    hash(hash::ACTION_TREE, [left, right])
}

#[test]
fn action_tree_root_depends_on_the_order_of_the_tags() {
    assert(action_tree_root([1, 2]) != action_tree_root([2, 1]));
}

#[test]
fn action_tree_root_pads_the_tags_with_padding_leaves() {
    assert_eq(action_tree_root([1, 2]), action_tree_root([1, 2, PADDING_LEAF]));
}

#[test]
fn root_of_a_path_is_the_action_tree_root() {
    let tags = [11, 12, 13, 14, 15];
    for index in 0..5 {
        assert_eq(action_tree_path(tags, index).root(tags[index]), action_tree_root(tags));
    }
}

#[test]
fn action_tree_path_holds_the_siblings_from_the_leaf_up() {
    let leaves = [11, 12, 13, 14, 15, PADDING_LEAF, PADDING_LEAF, PADDING_LEAF];
    let path = action_tree_path([11, 12, 13, 14, 15], 2);
    assert_eq(path.siblings[0], leaves[3]);
    assert_eq(path.siblings[1], node_hash(leaves[0], leaves[1]));
    assert_eq(
        path.siblings[2],
        node_hash(
            node_hash(leaves[4], leaves[5]),
            node_hash(leaves[6], leaves[7]),
        ),
    );
}

#[test(should_fail)]
fn root_fails_on_a_position_past_the_leaves() {
    let path = ActionTreePath { siblings: [0; ACTION_TREE_DEPTH], index: MAX_RESOURCES };
    let _ = path.root(1);
}
