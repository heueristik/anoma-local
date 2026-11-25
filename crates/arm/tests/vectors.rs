//! Values that the Noir library and the contract compute for the same inputs.

use anoma_arm::action::{Action, ResourceInstance, action_tree_root};
use anoma_arm::app_data::{AppData, Blob, DeletionCriterion};
use anoma_arm::belt::{Belt, node_hash};
use anoma_arm::delta::{ResourceDelta, unit_delta};
use anoma_arm::digest::{Deployment, extend};
use anoma_arm::field::{from_bytes, scalar_limbs, to_bytes, to_hex};
use anoma_arm::hash::poseidon2_hash;
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_arm::{Fr, Point, Scalar};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::PrimeField;

fn fr(hex: &str) -> Fr {
    let bytes: [u8; 32] = hex::decode(hex.trim_start_matches("0x"))
        .expect("the vector is hexadecimal")
        .try_into()
        .expect("the vector has 32 bytes");
    from_bytes(bytes).expect("the vector is a field element")
}

#[test]
fn poseidon2_hash_matches_noir() {
    assert_eq!(
        poseidon2_hash(&[Fr::from(1000u64), Fr::from(2000u64)]),
        fr("0x118d5a5ecb25dafe99eb45cb196604a23d0b7c0cbd0c2be29e0787e59b7a1d8a")
    );
}

#[test]
fn node_hash_matches_the_contract_and_noir() {
    let expected = fr("0x2437d9514d36909ae8e6c80bb839bd22a8ad60d9b31d2ce92392151ec5d2aba9");
    assert_eq!(node_hash(Fr::from(1u64), Fr::from(2u64)), expected);

    let mut belt = Belt::default();
    belt.append(&[Fr::from(1u64), Fr::from(2u64)]);
    assert_eq!(belt.root(), expected);
}

#[test]
fn commitment_and_nullifier_match_noir() {
    let key = NullifierKey(Fr::from(42u64));
    let resource = Resource {
        logic_ref: Fr::from(1u64),
        label_ref: Fr::from(2u64),
        value_ref: Fr::from(3u64),
        quantity: 4,
        nonce: Fr::from(5u64),
        nullifier_key_commitment: key.commitment(),
        is_ephemeral: false,
        rand_seed: Fr::from(6u64),
    };

    assert_eq!(
        resource.commitment(),
        fr("0x2a624eb9644481627292ed4246f710962085375bc1c415081812f39ab59075e2")
    );
    assert_eq!(
        resource.nullifier(key),
        Ok(fr(
            "0x278a0152ca2158741a4f8e2f39c5554b737c6aab6ef2fe76e82b3cd492297e13"
        ))
    );
    assert_eq!(
        created_nonce(1, &[Fr::from(7u64), Fr::from(8u64)]),
        fr("0x25e80bbd6ce26324007452bf09f53eca227e51b602da3ef8d841c2e3dc2d6174")
    );
    assert_eq!(
        action_tree_root(&[Fr::from(1u64), Fr::from(2u64)]),
        Ok(fr(
            "0x1c6eb2d928178f569eff3997f1d9ab11069c403f16f842273d7d397f033b62dc"
        ))
    );
}

#[test]
fn unit_delta_matches_noir() {
    let kind = (Point::generator() + Point::generator()).into_affine();
    let share = |quantity, is_consumed| ResourceDelta {
        kind_base: kind,
        quantity,
        is_consumed,
    };
    let delta = unit_delta(
        &[share(10, true), share(3, false), share(4, false)],
        Scalar::from(99u64),
    );

    assert_eq!(
        delta.x,
        fr("0x1ecb26016f6aef189bfd215d3cbd114f0f74edf07f296d5762702003482d9dac")
    );
    assert_eq!(
        delta.y,
        fr("0x065fab200eda11ff5267a0045eeb0a599c8ea1b5548c907f95c8df351c9964c9")
    );
}

#[test]
fn scalar_limbs_match_noir() {
    // Noir's `EmbeddedCurveScalar::from_field(-1)`.
    let minus_one = Scalar::from_be_bytes_mod_order(&to_bytes(-Fr::from(1u64)));
    let (lo, hi) = scalar_limbs(minus_one);
    assert_eq!(
        to_hex(lo),
        format!("0x{:0>64}", "2833e84879b9709143e1f593f0000000")
    );
    assert_eq!(
        to_hex(hi),
        format!("0x{:0>64}", "30644e72e131a029b85045b68181585d")
    );
}

#[test]
fn bind_matches_the_contract_and_noir() {
    let mut verifier = [0; 20];
    verifier[19] = 2;
    let deployment = Deployment {
        chain_id: 1,
        verifier,
    };

    assert_eq!(
        deployment.bind(Fr::from(3u64)),
        fr("0x1701f34c80e1ef7f8125e5d4d2d7e19b509e25d26e462d5308b5abb95b64783e")
    );
}

#[test]
fn extend_matches_the_contract_and_noir() {
    let empty = AppData::default().hash();
    assert_eq!(
        empty,
        fr("0x174708fff7719dd5979ec875d56cd2286f6d3cf7ec317a3b25632aab28ec37bb")
    );

    let resource =
        |is_consumed, tag: u64, logic_ref: u64, commitment_tree_root: u64| ResourceInstance {
            is_consumed,
            tag: Fr::from(tag),
            logic_ref: Fr::from(logic_ref),
            commitment_tree_root: Fr::from(commitment_tree_root),
            app_data: AppData::default(),
        };
    let action = Action {
        resources: vec![resource(true, 1, 2, 3), resource(false, 4, 5, 0)],
    };
    assert_eq!(
        extend(Fr::from(0u64), &action),
        fr("0x0b60bead17f79a743fd35dc5f1d9948be77dceede95f75c6a7de00414a41138d")
    );
}

#[test]
fn app_data_hash_packs_each_blob_with_its_criterion_and_length() {
    let app_data = AppData {
        resource_payload: vec![Blob {
            deletion_criterion: DeletionCriterion::Never,
            bytes: b"ab".to_vec(),
        }],
        application_payload: vec![Blob {
            deletion_criterion: DeletionCriterion::Immediately,
            bytes: b"c".to_vec(),
        }],
        ..AppData::default()
    };
    let packed = [
        &[0, 0, 0, 1, 1, 0, 0, 0, 2, b'a', b'b'][..],
        &[0, 0, 0, 0],
        &[0, 0, 0, 0],
        &[0, 0, 0, 1, 0, 0, 0, 0, 1, b'c'],
    ]
    .concat();
    let mut hash: [u8; 32] = <sha2::Sha256 as sha2::Digest>::digest(&packed).into();
    hash[0] &= 0x1f;

    assert_eq!(
        app_data.hash(),
        from_bytes(hash).expect("253 bits are a field element")
    );
}
