//! An action as the contract takes it, and its action tree: a Poseidon2 Merkle tree of fixed depth over its tags.

use ark_ff::AdditiveGroup;

use crate::app_data::AppData;
use crate::hash::{Domain, hash};
use crate::{Error, Fr};

/// The largest number of resources of an action: the number of leaves of its action tree.
pub const MAX_RESOURCES: usize = 8;

/// The depth of an action tree, so that a path has the same length for each size of action.
pub const ACTION_TREE_DEPTH: usize = 3;

/// The leaves after the last tag of an action; no tag is zero, since each tag is a Poseidon2 hash.
pub const PADDING_LEAF: Fr = Fr::ZERO;

/// One resource of an action as the contract takes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceInstance {
    pub is_consumed: bool,
    /// The nullifier of a consumed resource or the commitment of a created resource.
    pub tag: Fr,
    pub logic_ref: Fr,
    /// The root that the membership proof of a consumed resource uses; zero for a created resource.
    pub commitment_tree_root: Fr,
    pub app_data: AppData,
}

/// An action as the contract takes it: one compliance unit of 1 to `MAX_RESOURCES` resources, of which at least one
/// is consumed, in the order of the unit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub resources: Vec<ResourceInstance>,
}

impl Action {
    pub fn tags(&self) -> Vec<Fr> {
        self.resources.iter().map(|resource| resource.tag).collect()
    }

    /// The tags of the consumed resources, in the order of the action.
    pub fn nullifiers(&self) -> impl Iterator<Item = Fr> + '_ {
        self.resources
            .iter()
            .filter(|resource| resource.is_consumed)
            .map(|resource| resource.tag)
    }

    /// The tags of the created resources, in the order of the action.
    pub fn commitments(&self) -> impl Iterator<Item = Fr> + '_ {
        self.resources
            .iter()
            .filter(|resource| !resource.is_consumed)
            .map(|resource| resource.tag)
    }
}

/// The root of the action tree over the tags of an action, in its order, with padding leaves after them.
pub fn action_tree_root(tags: &[Fr]) -> Result<Fr, Error> {
    Ok(action_tree_nodes(tags)?[0])
}

/// The path of the tag at `index` in the action tree over `tags`.
pub fn action_tree_path(tags: &[Fr], index: usize) -> Result<ActionTreePath, Error> {
    if index >= tags.len() {
        return Err(Error::NotInAction {
            index,
            resource_count: tags.len(),
        });
    }
    let nodes = action_tree_nodes(tags)?;
    let mut siblings = [PADDING_LEAF; ACTION_TREE_DEPTH];
    let mut node = MAX_RESOURCES - 1 + index;
    for sibling in &mut siblings {
        // A left child has an odd index, so its sibling is the next node.
        *sibling = nodes[if node % 2 == 1 { node + 1 } else { node - 1 }];
        node = (node - 1) / 2;
    }
    Ok(ActionTreePath { siblings, index })
}

/// The path of a leaf of an action tree: the siblings from the leaf up, and the position of the leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionTreePath {
    pub siblings: [Fr; ACTION_TREE_DEPTH],
    pub index: usize,
}

impl ActionTreePath {
    /// Computes the root from the leaf at the position of the path.
    pub fn root(&self, leaf: Fr) -> Fr {
        self.siblings
            .iter()
            .enumerate()
            .fold(leaf, |node, (level, &sibling)| {
                if (self.index >> level) & 1 == 1 {
                    node_hash(sibling, node)
                } else {
                    node_hash(node, sibling)
                }
            })
    }
}

/// The nodes of the action tree over `tags`. Node i has the children 2i + 1 and 2i + 2, so the leaves are the last
/// `MAX_RESOURCES` nodes.
fn action_tree_nodes(tags: &[Fr]) -> Result<[Fr; 2 * MAX_RESOURCES - 1], Error> {
    if tags.len() > MAX_RESOURCES {
        return Err(Error::TooManyResources {
            resource_count: tags.len(),
            max: MAX_RESOURCES,
        });
    }
    let mut nodes = [PADDING_LEAF; 2 * MAX_RESOURCES - 1];
    nodes[MAX_RESOURCES - 1..][..tags.len()].copy_from_slice(tags);
    for i in (0..MAX_RESOURCES - 1).rev() {
        nodes[i] = node_hash(nodes[2 * i + 1], nodes[2 * i + 2]);
    }
    Ok(nodes)
}

fn node_hash(left: Fr, right: Fr) -> Fr {
    hash(Domain::ActionTree, &[left, right])
}
