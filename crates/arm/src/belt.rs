//! The commitment tree: a Merkle Mountain Belt over the commitments, with the root that the contract computes and
//! the membership paths that the compliance circuit checks.

use ark_ff::AdditiveGroup;

use crate::hash::{Domain, hash};
use crate::{Error, Fr};

/// The largest path length in the compliance circuit, which covers the newest 2^32 leaves.
pub const MAX_PATH_LENGTH: usize = 67;

/// All leaves of a belt, in their order; the nodes come from the leaves at each use.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Belt {
    leaves: Vec<Fr>,
}

/// The siblings from a leaf up to the root, each with its side.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MembershipPath {
    pub siblings: Vec<Fr>,
    pub sibling_is_left: Vec<bool>,
}

impl MembershipPath {
    pub fn root(&self, leaf: Fr) -> Fr {
        self.siblings
            .iter()
            .zip(&self.sibling_is_left)
            .fold(leaf, |node, (&sibling, &is_left)| {
                if is_left {
                    node_hash(sibling, node)
                } else {
                    node_hash(node, sibling)
                }
            })
    }
}

impl Belt {
    pub fn append(&mut self, leaves: &[Fr]) {
        self.leaves.extend_from_slice(leaves);
    }

    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }

    /// The root, or zero for the belt without leaves.
    pub fn root(&self) -> Fr {
        let tree = Tree::new(&self.leaves);
        tree.root.map_or(Fr::ZERO, |root| tree.nodes[root].hash)
    }

    /// The membership path of the leaf at `index`.
    pub fn path(&self, index: usize) -> Result<MembershipPath, Error> {
        let tree = Tree::new(&self.leaves);
        let Some(&leaf) = tree.leaves.get(index) else {
            return Err(Error::LeafNotInBelt {
                index,
                leaf_count: self.leaves.len(),
            });
        };
        let mut path = MembershipPath {
            siblings: Vec::new(),
            sibling_is_left: Vec::new(),
        };
        let mut node = leaf;
        while let Some(parent) = tree.nodes[node].parent {
            let (left, right) = tree.nodes[parent]
                .children
                .expect("a parent has two children");
            let is_left = right == node;
            path.siblings
                .push(tree.nodes[if is_left { left } else { right }].hash);
            path.sibling_is_left.push(is_left);
            node = parent;
        }
        if path.siblings.len() > MAX_PATH_LENGTH {
            return Err(Error::PathTooLong {
                index,
                length: path.siblings.len(),
                max: MAX_PATH_LENGTH,
            });
        }
        Ok(path)
    }
}

/// The hash of two nodes, which the contract computes as `Poseidon2T4.hash3(9, left, right)`.
pub fn node_hash(left: Fr, right: Fr) -> Fr {
    hash(Domain::BeltNode, &[left, right])
}

struct Node {
    hash: Fr,
    parent: Option<usize>,
    children: Option<(usize, usize)>,
}

/// The nodes of a belt in one list: the leaves, the mountains, the ranges and the belt.
struct Tree {
    nodes: Vec<Node>,
    leaves: Vec<usize>,
    root: Option<usize>,
}

impl Tree {
    fn new(leaves: &[Fr]) -> Self {
        let mut tree = Self {
            nodes: Vec::new(),
            leaves: Vec::new(),
            root: None,
        };
        // The peaks from the oldest to the newest, each with its height.
        let mut mountains: Vec<(usize, u32)> = Vec::new();
        for (index, &leaf) in leaves.iter().enumerate() {
            let node = tree.push(leaf, None);
            tree.leaves.push(node);
            mountains.push((node, 0));

            // Step n merges the rightmost two mountains of height ν2(n + 1), unless n + 1 is a power of two.
            let number = index + 1;
            let next = number + 1;
            if next & number != 0 {
                let height = next.trailing_zeros();
                let right = (1..mountains.len())
                    .rev()
                    .find(|&right| mountains[right].1 == height && mountains[right - 1].1 == height)
                    .expect("step n has two mountains of height ν2(n + 1)");
                let merged = tree.join(mountains[right - 1].0, mountains[right].0);
                mountains[right - 1] = (merged, height + 1);
                mountains.remove(right);
            }
        }
        tree.root = tree.bag(leaves.len(), &mountains);
        tree
    }

    /// Joins the peaks into ranges and the ranges into the belt, from the oldest mountain on.
    fn bag(&mut self, leaf_count: usize, mountains: &[(usize, u32)]) -> Option<usize> {
        let (&(first, _), rest) = mountains.split_first()?;
        let bits = leaf_count + 1;
        let count = mountains.len();
        let mut belt = None;
        let mut range = first;
        for (offset, &(peak, _)) in rest.iter().enumerate() {
            let i = offset + 1;
            // Mountain i starts a range if the bit before it is one, and its own bit or the bit two before it is zero.
            let bit = (bits >> (count - 1 - i)) & 1;
            let previous = (bits >> (count - i)) & 1;
            let two_before = if i >= 2 {
                (bits >> (count + 1 - i)) & 1
            } else {
                1
            };
            if previous == 1 && (bit == 0 || two_before == 0) {
                belt = Some(belt.map_or(range, |belt| self.join(belt, range)));
                range = peak;
            } else {
                range = self.join(range, peak);
            }
        }
        Some(belt.map_or(range, |belt| self.join(belt, range)))
    }

    fn push(&mut self, hash: Fr, children: Option<(usize, usize)>) -> usize {
        self.nodes.push(Node {
            hash,
            parent: None,
            children,
        });
        self.nodes.len() - 1
    }

    fn join(&mut self, left: usize, right: usize) -> usize {
        let parent = self.push(
            node_hash(self.nodes[left].hash, self.nodes[right].hash),
            Some((left, right)),
        );
        self.nodes[left].parent = Some(parent);
        self.nodes[right].parent = Some(parent);
        parent
    }
}
