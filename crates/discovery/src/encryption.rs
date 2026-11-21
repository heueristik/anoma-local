//! ECIES on Grumpkin with Poseidon2, over BN254 field elements, which a BN254 circuit computes natively. The key is the
//! hash of the shared point, a keystream of hashes of the key masks the message, and a hash of the key and the message
//! is the tag. The scalar multiplication of arkworks does not take constant time, so a party that measures the time of
//! many decryptions can learn bits of the secret key.

use anoma_arm::field::{coordinates, from_bytes, point_from_coordinates, to_bytes};
use anoma_arm::hash::{Domain, hash};
use anoma_arm::{Fr, Point, Scalar};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{AdditiveGroup, UniformRand};
use rand_core::OsRng;

/// A Grumpkin public key, which is never the point at infinity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicKey(Point);

/// A nonzero Grumpkin secret key, with its public key.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretKey {
    scalar: Scalar,
    public_key: PublicKey,
}

/// A message encrypted to a receiver: the public key of the sender's ephemeral key, the tag, and each field element of
/// the message, masked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ciphertext {
    pub ephemeral_key: PublicKey,
    pub tag: Fr,
    pub masked: Vec<Fr>,
}

/// Encrypts a message to a receiver with the sender's ephemeral key, which the sender uses for one message only.
pub fn encrypt(message: &[Fr], receiver: &PublicKey, ephemeral_key: &SecretKey) -> Ciphertext {
    let key = shared_key(receiver, ephemeral_key);
    Ciphertext {
        ephemeral_key: ephemeral_key.public_key,
        tag: tag(key, message),
        masked: message
            .iter()
            .zip(keystream(key))
            .map(|(&field, mask)| field + mask)
            .collect(),
    }
}

/// The message of the ciphertext, if the tag shows that it is for the receiver's secret key.
pub fn decrypt(ciphertext: &Ciphertext, receiver: &SecretKey) -> Option<Vec<Fr>> {
    let key = shared_key(&ciphertext.ephemeral_key, receiver);
    let message: Vec<Fr> = ciphertext
        .masked
        .iter()
        .zip(keystream(key))
        .map(|(&masked, mask)| masked - mask)
        .collect();
    (tag(key, &message) == ciphertext.tag).then_some(message)
}

impl SecretKey {
    pub fn random() -> Self {
        loop {
            let scalar = Scalar::rand(&mut OsRng);
            if scalar != Scalar::ZERO {
                return Self {
                    scalar,
                    public_key: PublicKey((Point::generator() * scalar).into_affine()),
                };
            }
        }
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }
}

impl PublicKey {
    /// The key of the coordinates, if they are a point on the curve.
    pub fn from_coordinates(x: Fr, y: Fr) -> Option<Self> {
        point_from_coordinates(x, y).map(Self)
    }

    pub fn coordinates(&self) -> [Fr; 2] {
        let (x, y) = coordinates(self.0);
        [x, y]
    }
}

impl Ciphertext {
    /// The ephemeral key, the tag and the masked message, each field element as 32 big-endian bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let [x, y] = self.ephemeral_key.coordinates();
        [x, y, self.tag]
            .iter()
            .chain(&self.masked)
            .flat_map(|&field| to_bytes(field))
            .collect()
    }

    /// The ciphertext of bytes that `to_bytes` wrote, or `None` for other bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let (words, rest) = bytes.as_chunks::<32>();
        if !rest.is_empty() {
            return None;
        }
        let fields = words
            .iter()
            .map(|&word| from_bytes(word).ok())
            .collect::<Option<Vec<Fr>>>()?;
        let (&[x, y, tag], masked) = fields.split_first_chunk::<3>()?;
        Some(Self {
            ephemeral_key: PublicKey::from_coordinates(x, y)?,
            tag,
            masked: masked.to_vec(),
        })
    }
}

/// The hash of the shared point, which only the two parties compute. The point is never at infinity, as both keys are
/// nonzero and the group has prime order.
fn shared_key(public_key: &PublicKey, secret_key: &SecretKey) -> Fr {
    let shared = (public_key.0 * secret_key.scalar).into_affine();
    let (x, y) = coordinates(shared);
    hash(Domain::SharedKey, &[x, y])
}

/// The masks of the field elements of a message, one for each index.
fn keystream(key: Fr) -> impl Iterator<Item = Fr> {
    (0u64..).map(move |index| hash(Domain::Keystream, &[key, Fr::from(index)]))
}

fn tag(key: Fr, message: &[Fr]) -> Fr {
    let preimage: Vec<Fr> = std::iter::once(key)
        .chain(message.iter().copied())
        .collect();
    hash(Domain::CiphertextTag, &preimage)
}
