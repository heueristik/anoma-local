//! The discovery of resources, as Anoma Galileo does it. The creator of a resource adds two blobs to its app data: a
//! discovery payload, which only the receiver's discovery key decrypts, and the resource plaintext encrypted to the
//! receiver's encryption key. A receiver finds its resources by trial decryption of all discovery payloads.

pub mod encryption;

use anoma_arm::digest::{AppData, Blob, DeletionCriterion};

pub use encryption::{Error, PublicKey, SecretKey};

/// The plaintext of a discovery payload; only the receiver's ability to decrypt it counts.
pub const DISCOVERY_MESSAGE: [u8; 1] = [0];

/// The public keys of a receiver: one finds its resources, the other reads them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Receiver {
    pub discovery_key: PublicKey,
    pub encryption_key: PublicKey,
}

/// The secret keys of a receiver.
#[derive(Clone, PartialEq, Eq)]
pub struct ReceiverKeys {
    pub discovery_key: SecretKey,
    pub encryption_key: SecretKey,
}

impl ReceiverKeys {
    pub fn random() -> Self {
        Self {
            discovery_key: SecretKey::random(),
            encryption_key: SecretKey::random(),
        }
    }

    pub fn receiver(&self) -> Receiver {
        Receiver {
            discovery_key: self.discovery_key.public_key(),
            encryption_key: self.encryption_key.public_key(),
        }
    }
}

/// The app data of a created resource for a receiver, which the chain emits: a discovery payload and the encrypted
/// plaintext.
pub fn app_data(receiver: &Receiver, plaintext: &[u8]) -> Result<AppData, Error> {
    let blob = |bytes| Blob {
        deletion_criterion: DeletionCriterion::Never,
        bytes,
    };
    Ok(AppData {
        resource_payload: vec![blob(encryption::encrypt(
            plaintext,
            &receiver.encryption_key,
        )?)],
        discovery_payload: vec![blob(encryption::encrypt(
            &DISCOVERY_MESSAGE,
            &receiver.discovery_key,
        )?)],
        ..AppData::default()
    })
}

/// The decrypted resource payloads of a created resource, if one of its discovery payloads is for the keys.
pub fn discover(app_data: &AppData, keys: &ReceiverKeys) -> Option<Vec<Vec<u8>>> {
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
