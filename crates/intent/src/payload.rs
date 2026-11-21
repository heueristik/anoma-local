//! What a party gives the solvers with an intent, in the resource payload of the intent resource.

use anoma_arm::delta::unit_delta;
use anoma_arm::field::{scalar_from_limbs, scalar_limbs, to_u128};
use anoma_arm::kind::Kind;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::ReceiverAddress;
use anoma_prover::PartialTransaction;
use anoma_prover::action::ProvedAction;

use crate::Terms;

/// The kind and quantity that the actions of an intent give up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Offer {
    pub logic_ref: Fr,
    pub label_ref: Fr,
    pub quantity: u128,
}

/// The fields of the intent that its terms do not fix, what the actions give up, the rcv sum that opens their delta,
/// and the address of the receiver of the resource that fills the terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Payload<A> {
    pub nonce: Fr,
    pub rand_seed: Fr,
    pub nullifier_key: NullifierKey,
    pub terms: Terms,
    pub offer: Offer,
    pub rcv_total: Scalar,
    pub receiver: A,
}

/// The number of field elements of an encoded payload without the address of the receiver, which comes last.
const FIXED_FIELDS: usize = 14;

/// Why a payload does not describe the actions of its intent.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Mismatch {
    #[error("no action creates the intent")]
    NotCreated,
    #[error("the delta of the actions is not the offer minus the intent")]
    Delta,
}

impl<A: ReceiverAddress> Payload<A> {
    /// The fields in the order of the struct, the rcv sum as its low and high 128 bits, and the address last.
    pub fn to_fields(&self) -> Vec<Fr> {
        let (rcv_low, rcv_high) = scalar_limbs(self.rcv_total);
        let mut fields = vec![
            self.nonce,
            self.rand_seed,
            self.nullifier_key.0,
            self.terms.want_logic_ref,
            self.terms.want_label_ref,
            self.terms.want_value_ref,
            Fr::from(self.terms.want_quantity),
            self.terms.receiver_nullifier_key_commitment,
            self.terms.want_rand_seed,
            self.offer.logic_ref,
            self.offer.label_ref,
            Fr::from(self.offer.quantity),
            rcv_low,
            rcv_high,
        ];
        fields.extend(self.receiver.to_fields());
        fields
    }

    /// The payload of field elements that `to_fields` wrote, or `None` for others.
    pub fn from_fields(fields: &[Fr]) -> Option<Self> {
        let (fixed, receiver) = fields.split_first_chunk::<FIXED_FIELDS>()?;
        let [
            nonce,
            rand_seed,
            nullifier_key,
            want_logic_ref,
            want_label_ref,
            want_value_ref,
            want_quantity,
            receiver_nullifier_key_commitment,
            want_rand_seed,
            offer_logic_ref,
            offer_label_ref,
            offer_quantity,
            rcv_low,
            rcv_high,
        ] = *fixed;
        Some(Self {
            nonce,
            rand_seed,
            nullifier_key: NullifierKey(nullifier_key),
            terms: Terms {
                want_logic_ref,
                want_label_ref,
                want_value_ref,
                want_quantity: to_u128(want_quantity)?,
                receiver_nullifier_key_commitment,
                want_rand_seed,
            },
            offer: Offer {
                logic_ref: offer_logic_ref,
                label_ref: offer_label_ref,
                quantity: to_u128(offer_quantity)?,
            },
            rcv_total: scalar_from_limbs(rcv_low, rcv_high)?,
            receiver: A::from_fields(receiver)?,
        })
    }
}

impl<A> Payload<A> {
    /// The intent of the payload, with `intent_logic_ref` as the key hash of the intent logic.
    pub fn intent(&self, intent_logic_ref: Fr) -> Resource {
        self.terms.intent(
            intent_logic_ref,
            self.nullifier_key,
            self.nonce,
            self.rand_seed,
        )
    }

    /// Checks that the payload describes the actions: they create the intent, and their delta is the offer minus
    /// the intent, opened by the rcv sum. Returns the actions as the partial transaction that the solver composes.
    pub fn check(
        &self,
        actions: &[ProvedAction],
        intent_logic_ref: Fr,
    ) -> Result<PartialTransaction, Mismatch> {
        let intent = self.intent(intent_logic_ref);
        let commitment = intent.commitment();
        if !actions
            .iter()
            .any(|proved| proved.action.commitment == commitment)
        {
            return Err(Mismatch::NotCreated);
        }
        let partial = PartialTransaction {
            actions: actions.to_vec(),
            rcv_total: self.rcv_total,
        };
        let expected = unit_delta(
            Kind::new(self.offer.logic_ref, self.offer.label_ref).base,
            self.offer.quantity,
            Kind::new(intent.logic_ref, intent.label_ref).base,
            intent.quantity,
            self.rcv_total,
        );
        if partial.delta() != expected {
            return Err(Mismatch::Delta);
        }
        Ok(partial)
    }
}
