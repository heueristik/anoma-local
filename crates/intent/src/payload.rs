//! What a party gives the solvers with an intent, in the resource payload of the intent resource.

use anoma_arm::delta::unit_delta;
use anoma_arm::field::{from_bytes, to_bytes};
use anoma_arm::kind::Kind;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::{PublicKey, Receiver};
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
/// and the keys that receive the resource that fills the terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Payload {
    pub nonce: Fr,
    pub rand_seed: Fr,
    pub nullifier_key: NullifierKey,
    pub terms: Terms,
    pub offer: Offer,
    pub rcv_total: Scalar,
    pub receiver: Receiver,
}

/// The length of an encoded payload.
const PAYLOAD_BYTES: usize = 3 * 32 + (5 * 32 + 16) + (2 * 32 + 16) + 32 + 2 * 33;

/// Why a payload does not describe the actions of its intent.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Mismatch {
    #[error("no action creates the intent")]
    NotCreated,
    #[error("the delta of the actions is not the offer minus the intent")]
    Delta,
}

impl Payload {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PAYLOAD_BYTES);
        for field in [
            self.nonce,
            self.rand_seed,
            self.nullifier_key.0,
            self.terms.want_logic_ref,
            self.terms.want_label_ref,
            self.terms.want_value_ref,
            self.terms.receiver_nullifier_key_commitment,
            self.terms.want_rand_seed,
        ] {
            bytes.extend_from_slice(&to_bytes(field));
        }
        bytes.extend_from_slice(&self.terms.want_quantity.to_be_bytes());
        for field in [self.offer.logic_ref, self.offer.label_ref] {
            bytes.extend_from_slice(&to_bytes(field));
        }
        bytes.extend_from_slice(&self.offer.quantity.to_be_bytes());
        bytes.extend_from_slice(&to_bytes(self.rcv_total));
        bytes.extend_from_slice(&self.receiver.discovery_key.to_bytes());
        bytes.extend_from_slice(&self.receiver.encryption_key.to_bytes());
        bytes
    }

    /// The payload of bytes that `to_bytes` wrote, or `None` for other bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut reader = Reader(bytes);
        let nonce = reader.field()?;
        let rand_seed = reader.field()?;
        let nullifier_key = NullifierKey(reader.field()?);
        let want_logic_ref = reader.field()?;
        let want_label_ref = reader.field()?;
        let want_value_ref = reader.field()?;
        let receiver_nullifier_key_commitment = reader.field()?;
        let want_rand_seed = reader.field()?;
        let want_quantity = u128::from_be_bytes(reader.take()?);
        let offer = Offer {
            logic_ref: reader.field()?,
            label_ref: reader.field()?,
            quantity: u128::from_be_bytes(reader.take()?),
        };
        let rcv_total = from_bytes(reader.take()?).ok()?;
        let receiver = Receiver {
            discovery_key: PublicKey::from_bytes(reader.take()?)?,
            encryption_key: PublicKey::from_bytes(reader.take()?)?,
        };
        reader.0.is_empty().then_some(Self {
            nonce,
            rand_seed,
            nullifier_key,
            terms: Terms {
                want_logic_ref,
                want_label_ref,
                want_value_ref,
                want_quantity,
                receiver_nullifier_key_commitment,
                want_rand_seed,
            },
            offer,
            rcv_total,
            receiver,
        })
    }

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

struct Reader<'a>(&'a [u8]);

impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> Option<[u8; N]> {
        let (chunk, rest) = self.0.split_first_chunk::<N>()?;
        self.0 = rest;
        Some(*chunk)
    }

    fn field(&mut self) -> Option<Fr> {
        from_bytes(self.take()?).ok()
    }
}
