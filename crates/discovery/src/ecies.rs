//! The discovery scheme of Anoma Galileo, with ECIES on secp256k1.

use anoma_arm::digest::{AppData, Blob, DeletionCriterion};

use crate::encryption::{self, Error, PublicKey, SecretKey};
use crate::{Discovery, ReceiverAddress};

/// The plaintext of a discovery payload; only the receiver's ability to decrypt it counts.
pub const DISCOVERY_MESSAGE: [u8; 1] = [0];

/// The discovery of Anoma Galileo, with ECIES on secp256k1. The app data has two blobs: a discovery payload, which only
/// the discovery key of the receiver decrypts, and the plaintext, encrypted to its encryption key. A receiver finds
/// its resources by trial decryption of all discovery payloads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EciesDiscovery;

/// The public keys of a receiver: one finds its resources, the other reads them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EciesAddress {
    pub discovery_key: PublicKey,
    pub encryption_key: PublicKey,
}

/// The secret keys of a receiver.
#[derive(Clone, PartialEq, Eq)]
pub struct EciesSecretKeys {
    pub discovery_key: SecretKey,
    pub encryption_key: SecretKey,
}

impl Discovery for EciesDiscovery {
    type Address = EciesAddress;
    type SecretKeys = EciesSecretKeys;
    type Error = Error;

    fn address(&self, secret_keys: &EciesSecretKeys) -> EciesAddress {
        secret_keys.address()
    }

    fn app_data(&self, address: &EciesAddress, plaintext: &[u8]) -> Result<AppData, Error> {
        let blob = |bytes| Blob {
            deletion_criterion: DeletionCriterion::Never,
            bytes,
        };
        Ok(AppData {
            resource_payload: vec![blob(encryption::encrypt(
                plaintext,
                &address.encryption_key,
            )?)],
            discovery_payload: vec![blob(encryption::encrypt(
                &DISCOVERY_MESSAGE,
                &address.discovery_key,
            )?)],
            ..AppData::default()
        })
    }

    fn discover(&self, app_data: &AppData, keys: &EciesSecretKeys) -> Option<Vec<Vec<u8>>> {
        let is_discovered = app_data.discovery_payload.iter().any(|blob| {
            encryption::decrypt(&blob.bytes, &keys.discovery_key)
                .is_ok_and(|message| message == DISCOVERY_MESSAGE)
        });
        is_discovered.then(|| {
            app_data
                .resource_payload
                .iter()
                .filter_map(|blob| encryption::decrypt(&blob.bytes, &keys.encryption_key).ok())
                .collect()
        })
    }
}

impl ReceiverAddress for EciesAddress {
    /// The compressed discovery key, then the compressed encryption key.
    fn to_bytes(&self) -> Vec<u8> {
        [
            self.discovery_key.to_bytes(),
            self.encryption_key.to_bytes(),
        ]
        .concat()
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let (discovery_key, encryption_key) = bytes.split_first_chunk::<33>()?;
        Some(Self {
            discovery_key: PublicKey::from_bytes(*discovery_key)?,
            encryption_key: PublicKey::from_bytes(encryption_key.try_into().ok()?)?,
        })
    }
}

impl EciesSecretKeys {
    pub fn random() -> Self {
        Self {
            discovery_key: SecretKey::random(),
            encryption_key: SecretKey::random(),
        }
    }

    pub fn address(&self) -> EciesAddress {
        EciesAddress {
            discovery_key: self.discovery_key.public_key(),
            encryption_key: self.encryption_key.public_key(),
        }
    }
}
