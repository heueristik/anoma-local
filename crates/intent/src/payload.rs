//! What a party gives the solvers with an intent, in the resource payload of the intent resource.

use anoma_arm::field::{from_bytes, to_bytes};
use anoma_arm::kind::Kind;
use anoma_arm::resource::{NullifierKey, RESOURCE_BYTES, Resource};
use anoma_arm::{Fr, Point, Scalar};
use anoma_discovery::{PublicKey, Receiver};
use anoma_prover::action::ProvedAction;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField};

use crate::Terms;

/// The kind and quantity that the actions of an intent give up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Offer {
    pub logic_ref: Fr,
    pub label_ref: Fr,
    pub quantity: u128,
}

/// The intent and what opens it, what the actions give up, the rcv sum that opens their delta, and the keys that
/// receive the resource that fills the terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Payload {
    pub intent: Resource,
    pub nullifier_key: NullifierKey,
    pub terms: Terms,
    pub offer: Offer,
    pub rcv_total: Scalar,
    pub receiver: Receiver,
}

/// The length of an encoded payload.
const PAYLOAD_BYTES: usize = RESOURCE_BYTES + 32 + (3 * 32 + 16) + (2 * 32 + 16) + 32 + 2 * 33;

/// Why a payload does not describe the actions of its intent.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Mismatch {
    #[error("the intent does not have the intent logic, the label of its terms and the quantity 1")]
    Intent,
    #[error("the nullifier key does not open the intent")]
    NullifierKey,
    #[error("no action creates the intent")]
    NotCreated,
    #[error("the delta of the actions is not the offer minus the intent")]
    Delta,
}

impl Payload {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PAYLOAD_BYTES);
        bytes.extend_from_slice(&self.intent.to_bytes());
        bytes.extend_from_slice(&to_bytes(self.nullifier_key.0));
        for field in [
            self.terms.want_logic_ref,
            self.terms.want_label_ref,
            self.terms.receiver_nullifier_key_commitment,
        ] {
            bytes.extend_from_slice(&to_bytes(field));
        }
        bytes.extend_from_slice(&self.terms.min_quantity.to_be_bytes());
        for field in [self.offer.logic_ref, self.offer.label_ref] {
            bytes.extend_from_slice(&to_bytes(field));
        }
        bytes.extend_from_slice(&self.offer.quantity.to_be_bytes());
        bytes.extend_from_slice(&self.rcv_total.into_bigint().to_bytes_be());
        bytes.extend_from_slice(&self.receiver.discovery_key.to_bytes());
        bytes.extend_from_slice(&self.receiver.encryption_key.to_bytes());
        bytes
    }

    /// The payload of bytes that `to_bytes` wrote, or `None` for other bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut reader = Reader(bytes);
        let intent = Resource::from_bytes(&reader.take()?).ok()?;
        let nullifier_key = NullifierKey(reader.field()?);
        let want_logic_ref = reader.field()?;
        let want_label_ref = reader.field()?;
        let receiver_nullifier_key_commitment = reader.field()?;
        let min_quantity = u128::from_be_bytes(reader.take()?);
        let offer = Offer {
            logic_ref: reader.field()?,
            label_ref: reader.field()?,
            quantity: u128::from_be_bytes(reader.take()?),
        };
        let rcv: [u8; 32] = reader.take()?;
        let rcv_total = Scalar::from_be_bytes_mod_order(&rcv);
        if rcv_total.into_bigint().to_bytes_be() != rcv {
            return None;
        }
        let receiver = Receiver {
            discovery_key: PublicKey::from_bytes(reader.take()?)?,
            encryption_key: PublicKey::from_bytes(reader.take()?)?,
        };
        reader.0.is_empty().then_some(Self {
            intent,
            nullifier_key,
            terms: Terms {
                want_logic_ref,
                want_label_ref,
                min_quantity,
                receiver_nullifier_key_commitment,
            },
            offer,
            rcv_total,
            receiver,
        })
    }

    /// Checks that the payload describes the actions: they create the intent, and their delta is the offer minus
    /// the intent, opened by the rcv sum. The solver can then fill the terms and keep the offer.
    pub fn check(&self, actions: &[ProvedAction], intent_logic_ref: Fr) -> Result<(), Mismatch> {
        let intent = &self.intent;
        if intent.logic_ref != intent_logic_ref
            || intent.label_ref != self.terms.hash()
            || intent.quantity != 1
        {
            return Err(Mismatch::Intent);
        }
        intent
            .nullifier(self.nullifier_key)
            .map_err(|_| Mismatch::NullifierKey)?;
        let commitment = intent.commitment();
        if !actions
            .iter()
            .any(|proved| proved.action.commitment == commitment)
        {
            return Err(Mismatch::NotCreated);
        }
        let delta = actions
            .iter()
            .fold(Point::zero().into_group(), |sum, proved| sum + proved.delta);
        let offer = Kind::new(self.offer.logic_ref, self.offer.label_ref).base;
        let expected = offer * Scalar::from(self.offer.quantity)
            - Kind::new(intent.logic_ref, intent.label_ref).base
            + Point::generator() * self.rcv_total;
        if delta.into_affine() != expected.into_affine() {
            return Err(Mismatch::Delta);
        }
        Ok(())
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
