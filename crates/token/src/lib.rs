//! The token resource: its label ref is the key hash of its issuer, and its value ref the key hash of its owner. The
//! action that consumes a token of a quantity carries a P-256 signature over its action tree root, with SHA-256 as
//! keriox signs data: of the owner for a persistent token, and of the issuer for an ephemeral token, whose consumption
//! mints the quantity.

use std::fmt::Debug;
use std::sync::Arc;

use anoma_arm::Fr;
use anoma_arm::field::to_bytes;
use anoma_arm::hash::{Domain, hash};
use anoma_arm::resource::Resource;
use anoma_prover::Error;
use anoma_prover::circuit_prover::CircuitInputs;
use anoma_prover::inputs::{self, Field, circuit_inputs};
use anoma_prover::resource_logic::{LogicContext, ResourceLogic};
use p256::ecdsa::signature::{self, Keypair};
use p256::ecdsa::{Signature, VerifyingKey};
use serde::Serialize;

/// The circuit of the token logic.
pub const CIRCUIT: &str = "token_logic";

/// A P-256 key that signs messages with ECDSA over SHA-256, as keriox does: a key in memory, or a key in a secure
/// element of the device.
pub trait Signer:
    signature::Signer<Signature> + Keypair<VerifyingKey = VerifyingKey> + Debug + Send + Sync
{
}

impl<T> Signer for T where
    T: signature::Signer<Signature> + Keypair<VerifyingKey = VerifyingKey> + Debug + Send + Sync
{
}

/// The hash of a P-256 public key, as the label ref of a token names its issuer and the value ref its owner.
pub fn public_key_hash(key: &VerifyingKey) -> Fr {
    let PublicKey { x, y } = PublicKey::from(key);
    let [x_high, x_low] = halves(x);
    let [y_high, y_low] = halves(y);
    hash(Domain::PublicKey, &[x_high, x_low, y_high, y_low])
}

/// The logic of a token, with the key that signs an action that consumes a token of a quantity.
#[derive(Clone, Debug, Default)]
pub struct TokenLogic {
    /// `None` is enough for a created token and for a consumed token of quantity 0.
    pub signer: Option<Arc<dyn Signer>>,
}

impl ResourceLogic for TokenLogic {
    fn circuit(&self) -> &str {
        CIRCUIT
    }

    fn private_inputs(&self, context: &LogicContext) -> Result<CircuitInputs, Error> {
        // The logic checks nothing for a created token, but nargo needs a value for each private input.
        let (token, nullifier_key) = if context.is_consumed {
            (context.action.consumed, context.action.nullifier_key.0)
        } else {
            (Resource::default(), Fr::from(0u64))
        };
        let (signer, signature) = match (&self.signer, context.is_consumed) {
            (Some(signer), true) => {
                let signature = signer
                    .try_sign(&to_bytes(context.action_tree_root))
                    .map_err(|source| Error::LogicInputs {
                        circuit: CIRCUIT.to_string(),
                        source: Box::new(source),
                    })?;
                // The circuit accepts only the low s of the two values that make a valid signature.
                let signature = signature.normalize_s().to_bytes().to_vec();
                (PublicKey::from(&signer.verifying_key()), signature)
            }
            _ => (
                PublicKey {
                    x: [0; 32],
                    y: [0; 32],
                },
                vec![0; 64],
            ),
        };
        circuit_inputs(
            CIRCUIT,
            &Witness {
                token: (&token).into(),
                nullifier_key: Field(nullifier_key),
                signer,
                signature,
            },
        )
    }
}

/// The private inputs of the token logic.
#[derive(Serialize)]
struct Witness {
    token: inputs::Resource,
    nullifier_key: Field,
    signer: PublicKey,
    signature: Vec<u8>,
}

/// A `PublicKey`: the big-endian bytes of the coordinates of a P-256 point.
#[derive(Serialize)]
struct PublicKey {
    x: [u8; 32],
    y: [u8; 32],
}

impl From<&VerifyingKey> for PublicKey {
    fn from(key: &VerifyingKey) -> Self {
        let point = key.to_sec1_point(false);
        let bytes = point.as_bytes();
        let coordinate = |range: std::ops::Range<usize>| -> [u8; 32] {
            bytes[range]
                .try_into()
                .expect("an uncompressed SEC1 point has a tag byte and two 32-byte coordinates")
        };
        Self {
            x: coordinate(1..33),
            y: coordinate(33..65),
        }
    }
}

/// The high and the low 16 of 32 big-endian bytes, each as a field element, as the token logic splits a coordinate.
fn halves(bytes: [u8; 32]) -> [Fr; 2] {
    let (high, low) = bytes.split_at(16);
    [high, low].map(|half| {
        Fr::from(u128::from_be_bytes(
            half.try_into().expect("a half has 16 bytes"),
        ))
    })
}
