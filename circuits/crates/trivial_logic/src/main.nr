/// A resource logic that accepts every resource, for test resources whose only owner check is the nullifier key.
fn main(
    tag: pub Field,
    is_consumed: pub bool,
    action_tree_root: pub Field,
    app_data_hash: pub Field,
) {
    // The public inputs bind the proof to one resource of one action; the logic checks nothing else.
    let _ = (tag, is_consumed, action_tree_root, app_data_hash);
}
