mod common;

use anoma_arm::Point;
use anoma_arm::field::coordinates;
use anoma_nostr_intent_pool::encoding::{from_bytes, to_bytes};
use ark_ec::AffineRepr;

/// The first resource of the first action follows the count of the actions and the count of its resources.
const FIRST_RESOURCE: usize = 8 + 8;

#[test]
fn from_bytes_reads_the_actions_of_to_bytes() {
    let mut larger = common::action(1);
    let created = larger.action.resources[1].clone();
    larger.action.resources.push(created);
    let logic_proof = larger.logic_proofs[1].clone();
    larger.logic_proofs.push(logic_proof);
    larger.delta = Point::zero();
    let actions = vec![common::action(0), larger];
    assert_ne!(
        actions[0].action.resources.len(),
        actions[1].action.resources.len(),
        "the actions have different numbers of resources"
    );

    assert_eq!(from_bytes(&to_bytes(&actions)), Some(actions));
}

#[test]
fn from_bytes_rejects_truncated_and_extended_bytes() {
    let bytes = to_bytes(&[common::action(0)]);
    assert!(from_bytes(&bytes).is_some(), "the bytes decode unchanged");

    assert_eq!(from_bytes(&bytes[..bytes.len() - 1]), None);
    assert_eq!(from_bytes(&[bytes.as_slice(), &[0]].concat()), None);
}

#[test]
fn from_bytes_rejects_a_flag_other_than_0_and_1() {
    let mut bytes = to_bytes(&[common::action(0)]);
    assert!(from_bytes(&bytes).is_some(), "the bytes decode unchanged");
    assert_eq!(bytes[FIRST_RESOURCE], 1, "the first resource is consumed");
    bytes[FIRST_RESOURCE] ^= 0xff;

    assert_eq!(from_bytes(&bytes), None);
}

#[test]
fn from_bytes_rejects_a_field_element_above_the_modulus() {
    let mut bytes = to_bytes(&[common::action(0)]);
    assert!(from_bytes(&bytes).is_some(), "the bytes decode unchanged");
    // The tag of the first resource follows its flag.
    bytes[FIRST_RESOURCE + 1..FIRST_RESOURCE + 1 + 32].fill(0xff);

    assert_eq!(from_bytes(&bytes), None);
}

#[test]
fn from_bytes_rejects_an_unknown_deletion_criterion() {
    let mut bytes = to_bytes(&[common::action(0)]);
    assert!(from_bytes(&bytes).is_some(), "the bytes decode unchanged");
    // The first blob of the first resource follows its flag, three field elements and the blob count.
    let criterion = FIRST_RESOURCE + 1 + 3 * 32 + 8;
    assert_eq!(bytes[criterion], 0, "the first blob is deleted immediately");
    bytes[criterion] ^= 0xff;

    assert_eq!(from_bytes(&bytes), None);
}

#[test]
fn from_bytes_rejects_a_delta_off_the_curve() {
    let mut action = common::action(0);
    let (x, y) = coordinates(action.delta);
    action.delta = Point::new_unchecked(x, y + y);
    assert!(!action.delta.is_on_curve(), "the delta is off the curve");

    assert_eq!(from_bytes(&to_bytes(&[action])), None);
}
