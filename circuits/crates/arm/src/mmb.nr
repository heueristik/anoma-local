use crate::hash::{self, hash};

/// The largest path length: a leaf among the newest 2^32 leaves of a Merkle Mountain Belt has a path of at most 2 * 32 + 3 nodes.
pub global MAX_PATH_LENGTH: u32 = 67;

/// A membership path in the Merkle Mountain Belt, padded to `MAX_PATH_LENGTH`, so that the proof does not show the age of the leaf.
pub struct MembershipPath {
    pub siblings: [Field; MAX_PATH_LENGTH],
    pub sibling_is_left: [bool; MAX_PATH_LENGTH],
    pub length: u32,
}

impl MembershipPath {
    /// Computes the root from the leaf; the hashes past `length` run but do not change the result.
    pub fn root(self, leaf: Field) -> Field {
        assert(self.length <= MAX_PATH_LENGTH, "the path is longer than the largest path length");
        let mut node = leaf;
        for i in 0..MAX_PATH_LENGTH {
            let parent = if self.sibling_is_left[i] {
                node_hash(self.siblings[i], node)
            } else {
                node_hash(node, self.siblings[i])
            };
            if i < self.length {
                node = parent;
            }
        }
        node
    }
}

/// The hash of two nodes of the belt, which the contract computes in the same way.
pub fn node_hash(left: Field, right: Field) -> Field {
    hash(hash::MMB_NODE, [left, right])
}

#[test]
fn node_hash_matches_the_commitment_tree_contract() {
    // The root of the contract's belt with the leaves 1 and 2.
    assert_eq(node_hash(1, 2), 0x2437d9514d36909ae8e6c80bb839bd22a8ad60d9b31d2ce92392151ec5d2aba9);
}

#[test]
fn root_of_an_empty_path_is_the_leaf() {
    let path = MembershipPath {
        siblings: [7; MAX_PATH_LENGTH],
        sibling_is_left: [false; MAX_PATH_LENGTH],
        length: 0,
    };
    assert_eq(path.root(5), 5);
}

#[test]
fn root_puts_each_sibling_on_its_side() {
    let mut path = MembershipPath {
        siblings: [0; MAX_PATH_LENGTH],
        sibling_is_left: [false; MAX_PATH_LENGTH],
        length: 2,
    };
    path.siblings[0] = 11;
    path.siblings[1] = 13;
    path.sibling_is_left[1] = true;
    assert_eq(path.root(5), node_hash(13, node_hash(5, 11)));
}

#[test(should_fail_with = "the path is longer than the largest path length")]
fn root_fails_on_a_path_that_is_too_long() {
    let path = MembershipPath {
        siblings: [0; MAX_PATH_LENGTH],
        sibling_is_left: [false; MAX_PATH_LENGTH],
        length: MAX_PATH_LENGTH + 1,
    };
    let _ = path.root(5);
}
