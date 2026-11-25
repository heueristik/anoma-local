use anoma_arm::Error;
use anoma_arm::Fr;
use anoma_arm::action::{MAX_RESOURCES, PADDING_LEAF, action_tree_path, action_tree_root};

fn tags(count: u64) -> Vec<Fr> {
    (11..11 + count).map(Fr::from).collect()
}

#[test]
fn action_tree_root_pads_the_tags_with_padding_leaves() {
    let mut padded = tags(2);
    padded.push(PADDING_LEAF);
    assert_eq!(action_tree_root(&tags(2)), action_tree_root(&padded));
}

#[test]
fn action_tree_root_errors_on_more_resources_than_leaves() {
    let count = MAX_RESOURCES + 1;
    assert_eq!(
        action_tree_root(&tags(count as u64)),
        Err(Error::TooManyResources {
            resource_count: count,
            max: MAX_RESOURCES,
        })
    );
}

#[test]
fn root_of_a_path_is_the_action_tree_root_for_each_index() -> Result<(), Error> {
    // The largest action has a leaf at each position, so it covers each path of the smaller actions.
    let tags = tags(MAX_RESOURCES as u64);
    for (index, &tag) in tags.iter().enumerate() {
        assert_eq!(
            action_tree_path(&tags, index)?.root(tag),
            action_tree_root(&tags)?
        );
    }
    Ok(())
}

#[test]
fn action_tree_path_errors_on_an_index_past_the_tags() {
    assert_eq!(
        action_tree_path(&tags(3), 3),
        Err(Error::NotInAction {
            index: 3,
            resource_count: 3,
        })
    );
}
