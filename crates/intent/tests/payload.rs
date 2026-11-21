use anoma_arm::delta::unit_delta;
use anoma_arm::digest::{Action, AppData};
use anoma_arm::kind::Kind;
use anoma_arm::resource::NullifierKey;
use anoma_arm::{Fr, Scalar};
use anoma_discovery::ReceiverKeys;
use anoma_intent::{Mismatch, Offer, Payload, Terms};
use anoma_prover::action::{LogicProof, ProvedAction};
use anoma_prover::toolchain::Proof;

const INTENT_LOGIC_REF: u64 = 9;

fn payload() -> Payload {
    let terms = Terms {
        want_logic_ref: Fr::from(1u64),
        want_label_ref: Fr::from(2u64),
        want_value_ref: Fr::from(13u64),
        want_quantity: 4,
        receiver_nullifier_key_commitment: NullifierKey(Fr::from(3u64)).commitment(),
        want_rand_seed: Fr::from(14u64),
    };
    let nullifier_key = NullifierKey(Fr::from(5u64));
    Payload {
        nonce: Fr::from(6u64),
        rand_seed: Fr::from(7u64),
        nullifier_key,
        terms,
        offer: Offer {
            logic_ref: Fr::from(1u64),
            label_ref: Fr::from(8u64),
            quantity: 4,
        },
        rcv_total: Scalar::from(11u64),
        receiver: ReceiverKeys::random().receiver(),
    }
}

/// The action that gives up `quantity` of the offered kind and creates the intent, with empty proofs.
fn giving_action(payload: &Payload, quantity: u128) -> ProvedAction {
    let intent = payload.intent(Fr::from(INTENT_LOGIC_REF));
    let offer = Kind::new(payload.offer.logic_ref, payload.offer.label_ref).base;
    let intent_kind = Kind::new(intent.logic_ref, intent.label_ref).base;
    ProvedAction {
        action: Action {
            nullifier: Fr::from(12u64),
            consumed_logic_ref: payload.offer.logic_ref,
            consumed_app_data: AppData::default(),
            commitment: intent.commitment(),
            created_logic_ref: intent.logic_ref,
            created_app_data: AppData::default(),
            commitment_tree_root: Fr::from(0u64),
        },
        delta: unit_delta(offer, quantity, intent_kind, 1, payload.rcv_total),
        compliance_proof: Proof::default(),
        consumed_logic_proof: LogicProof::default(),
        created_logic_proof: LogicProof::default(),
    }
}

#[test]
fn from_bytes_gives_the_payload_that_to_bytes_encoded() {
    let payload = payload();

    assert_eq!(Payload::from_bytes(&payload.to_bytes()), Some(payload));
}

#[test]
fn from_bytes_fails_on_an_extra_byte() {
    let mut bytes = payload().to_bytes();
    assert!(
        Payload::from_bytes(&bytes).is_some(),
        "the premise is that the encoding decodes"
    );

    bytes.push(0);

    assert_eq!(Payload::from_bytes(&bytes), None);
}

#[test]
fn check_gives_the_actions_that_give_up_the_offer_for_the_intent() -> Result<(), Mismatch> {
    let payload = payload();

    let actions = [giving_action(&payload, payload.offer.quantity)];

    let partial = payload.check(&actions, Fr::from(INTENT_LOGIC_REF))?;

    assert_eq!(partial.actions, actions);
    assert_eq!(partial.rcv_total, payload.rcv_total);
    Ok(())
}

#[test]
fn check_errors_on_an_action_that_gives_up_less_than_the_offer() {
    let payload = payload();

    let actions = [giving_action(&payload, payload.offer.quantity - 1)];

    assert_eq!(
        payload.check(&actions, Fr::from(INTENT_LOGIC_REF)),
        Err(Mismatch::Delta)
    );
}

#[test]
fn check_errors_on_actions_that_do_not_create_the_intent() {
    let payload = payload();
    let mut action = giving_action(&payload, payload.offer.quantity);

    action.action.commitment += Fr::from(1u64);

    assert_eq!(
        payload.check(&[action], Fr::from(INTENT_LOGIC_REF)),
        Err(Mismatch::NotCreated)
    );
}

#[test]
fn check_errors_on_an_intent_of_another_logic() {
    let payload = payload();
    let actions = [giving_action(&payload, payload.offer.quantity)];

    assert_eq!(
        payload.check(&actions, Fr::from(INTENT_LOGIC_REF + 1)),
        Err(Mismatch::NotCreated)
    );
}
