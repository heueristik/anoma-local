use anoma_arm::Error;
use anoma_arm::Fr;
use anoma_arm::belt::{Belt, node_hash};

fn belt(count: u64) -> Belt {
    let mut belt = Belt::default();
    belt.append(&(1..=count).map(Fr::from).collect::<Vec<_>>());
    belt
}

#[test]
fn root_of_the_empty_belt_is_zero() {
    assert_eq!(Belt::default().root(), Fr::from(0u64));
}

#[test]
fn root_of_eleven_leaves_joins_two_ranges() {
    // Three mountains of heights 3, 1 and 0, and the leaf count plus one, 0b1100, starts a range at the second one.
    let leaf = |n: u64| Fr::from(n);
    let left = node_hash(node_hash(leaf(1), leaf(2)), node_hash(leaf(3), leaf(4)));
    let right = node_hash(node_hash(leaf(5), leaf(6)), node_hash(leaf(7), leaf(8)));
    let expected = node_hash(
        node_hash(left, right),
        node_hash(node_hash(leaf(9), leaf(10)), leaf(11)),
    );

    assert_eq!(belt(11).root(), expected);
}

#[test]
fn path_leads_from_each_leaf_to_the_root_for_each_leaf_count_up_to_40() {
    for count in 1..=40 {
        let belt = belt(count);
        for index in 0..count as usize {
            let path = belt.path(index).expect("the leaf is in the belt");
            assert_eq!(
                path.root(Fr::from(index as u64 + 1)),
                belt.root(),
                "count {count} index {index}"
            );
        }
    }
}

#[test]
fn path_of_the_newest_leaf_has_at_most_three_nodes() {
    // The paper bounds the path of the k-th newest leaf by 2 * log2(k) + 3 nodes.
    for count in 1..=200 {
        let length = belt(count)
            .path(count as usize - 1)
            .expect("the leaf is in the belt")
            .siblings
            .len();
        assert!(length <= 3, "count {count} gives {length} nodes");
    }
}

#[test]
fn path_errors_on_a_leaf_outside_the_belt() {
    assert_eq!(
        belt(3).path(3),
        Err(Error::LeafNotInBelt {
            index: 3,
            leaf_count: 3
        })
    );
}
