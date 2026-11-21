//! ECIES on secp256k1, as Anoma Galileo encrypts payloads: the AES key is the SHA-256 hash of the receiver's public
//! key and the shared point, and AES-256-GCM encrypts the message.

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::sha2::{Digest, Sha256};
use k256::{AffinePoint, ProjectivePoint};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};

/// The errors of encryption and decryption.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the ciphertext is not a bincode envelope: {0}")]
    Envelope(#[from] bincode::Error),
    #[error("AES-GCM cannot encrypt a message of {0} bytes")]
    Encryption(usize),
    #[error("the key does not decrypt the ciphertext")]
    Decryption,
    #[error("the shared point is the point at infinity")]
    SharedPoint,
}

/// A secp256k1 secret key.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretKey(k256::SecretKey);

/// A secp256k1 public key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey(AffinePoint);

impl SecretKey {
    pub fn random() -> Self {
        Self(k256::SecretKey::random(&mut OsRng))
    }

    /// The secret key of 32 big-endian bytes, if they are a nonzero scalar below the group order.
    pub fn from_bytes(bytes: [u8; 32]) -> Option<Self> {
        k256::SecretKey::from_bytes(&bytes.into()).ok().map(Self)
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(*self.0.public_key().as_affine())
    }
}

impl PublicKey {
    /// The compressed SEC1 encoding.
    pub fn to_bytes(&self) -> [u8; 33] {
        let encoded = self.0.to_encoded_point(true);
        let mut bytes = [0; 33];
        bytes.copy_from_slice(encoded.as_bytes());
        bytes
    }

    /// The key of a compressed SEC1 encoding, if it is a point of the curve other than the point at infinity.
    pub fn from_bytes(bytes: [u8; 33]) -> Option<Self> {
        k256::PublicKey::from_sec1_bytes(&bytes)
            .ok()
            .map(|key| Self(*key.as_affine()))
    }
}

/// The ciphertext, the nonce and the sender's public key, as bincode encodes them.
#[derive(Serialize, Deserialize)]
struct Envelope {
    cipher: Vec<u8>,
    nonce: [u8; 12],
    sender: PublicKey,
}

/// Encrypts a message to a receiver with a new sender key and a random nonce.
pub fn encrypt(message: &[u8], receiver: &PublicKey) -> Result<Vec<u8>, Error> {
    let sender = SecretKey::random();
    let shared = ProjectivePoint::from(receiver.0) * sender.0.to_nonzero_scalar().as_ref();
    let cipher_key = aes(receiver, shared)?;
    let mut nonce = [0; 12];
    OsRng.fill_bytes(&mut nonce);
    let cipher = cipher_key
        .encrypt(&nonce.into(), message)
        .map_err(|_| Error::Encryption(message.len()))?;
    Ok(bincode::serialize(&Envelope {
        cipher,
        nonce,
        sender: sender.public_key(),
    })?)
}

/// Decrypts a ciphertext with the receiver's secret key.
pub fn decrypt(ciphertext: &[u8], receiver: &SecretKey) -> Result<Vec<u8>, Error> {
    let envelope: Envelope = bincode::deserialize(ciphertext)?;
    let shared = ProjectivePoint::from(envelope.sender.0) * receiver.0.to_nonzero_scalar().as_ref();
    aes(&receiver.public_key(), shared)?
        .decrypt(&envelope.nonce.into(), envelope.cipher.as_slice())
        .map_err(|_| Error::Decryption)
}

/// The AES-256-GCM cipher of the SHA-256 hash of the compressed receiver key and the compressed shared point.
fn aes(receiver: &PublicKey, shared: ProjectivePoint) -> Result<Aes256Gcm, Error> {
    let shared = shared.to_affine().to_encoded_point(true);
    if shared.is_identity() {
        return Err(Error::SharedPoint);
    }
    let key = Sha256::new()
        .chain_update(receiver.to_bytes())
        .chain_update(shared.as_bytes())
        .finalize();
    Ok(Aes256Gcm::new(&key))
}
