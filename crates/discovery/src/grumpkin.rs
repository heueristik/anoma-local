//! The discovery scheme of the local circuits, with ECIES on Grumpkin and Poseidon2.

use std::convert::Infallible;

use anoma_arm::Fr;
use anoma_arm::app_data::{AppData, Blob, DeletionCriterion};

use crate::encryption::{self, Ciphertext, PublicKey, SecretKey};
use crate::{Discovery, ReceiverAddress};

/// The discovery with ECIES on Grumpkin. The app data has two blobs: a discovery payload, an empty message whose tag
/// only the discovery key of the receiver checks, and the plaintext, encrypted to its encryption key. A receiver finds
/// its resources by trial decryption of all discovery payloads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GrumpkinDiscovery;

/// The public keys of a receiver: one finds its resources, the other reads them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GrumpkinAddress {
    pub discovery_key: PublicKey,
    pub encryption_key: PublicKey,
}

/// The secret keys of a receiver.
#[derive(Clone, PartialEq, Eq)]
pub struct GrumpkinSecretKeys {
    pub discovery_key: SecretKey,
    pub encryption_key: SecretKey,
}

impl Discovery for GrumpkinDiscovery {
    type Address = GrumpkinAddress;
    type SecretKeys = GrumpkinSecretKeys;
    type Error = Infallible;

    fn address(&self, secret_keys: &GrumpkinSecretKeys) -> GrumpkinAddress {
        secret_keys.address()
    }

    fn app_data(&self, address: &GrumpkinAddress, plaintext: &[Fr]) -> Result<AppData, Infallible> {
        let blob = |message: &[Fr], receiver: &PublicKey| Blob {
            deletion_criterion: DeletionCriterion::Never,
            bytes: encryption::encrypt(message, receiver, &SecretKey::random()).to_bytes(),
        };
        Ok(AppData {
            resource_payload: vec![blob(plaintext, &address.encryption_key)],
            discovery_payload: vec![blob(&[], &address.discovery_key)],
            ..AppData::default()
        })
    }

    fn discover(&self, app_data: &AppData, keys: &GrumpkinSecretKeys) -> Option<Vec<Vec<Fr>>> {
        let decrypt = |blob: &Blob, key: &SecretKey| {
            encryption::decrypt(&Ciphertext::from_bytes(&blob.bytes)?, key)
        };
        let is_discovered = app_data
            .discovery_payload
            .iter()
            .any(|blob| decrypt(blob, &keys.discovery_key).is_some());
        is_discovered.then(|| {
            app_data
                .resource_payload
                .iter()
                .filter_map(|blob| decrypt(blob, &keys.encryption_key))
                .collect()
        })
    }
}

impl ReceiverAddress for GrumpkinAddress {
    /// The coordinates of the discovery key, then the coordinates of the encryption key.
    fn to_fields(&self) -> Vec<Fr> {
        [
            self.discovery_key.coordinates(),
            self.encryption_key.coordinates(),
        ]
        .concat()
    }

    fn from_fields(fields: &[Fr]) -> Option<Self> {
        let &[discovery_x, discovery_y, encryption_x, encryption_y] = fields else {
            return None;
        };
        Some(Self {
            discovery_key: PublicKey::from_coordinates(discovery_x, discovery_y)?,
            encryption_key: PublicKey::from_coordinates(encryption_x, encryption_y)?,
        })
    }
}

impl GrumpkinSecretKeys {
    pub fn random() -> Self {
        Self {
            discovery_key: SecretKey::random(),
            encryption_key: SecretKey::random(),
        }
    }

    pub fn address(&self) -> GrumpkinAddress {
        GrumpkinAddress {
            discovery_key: self.discovery_key.public_key(),
            encryption_key: self.encryption_key.public_key(),
        }
    }
}
