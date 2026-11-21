//! The discovery of resources: the creator of a resource adds payloads to its app data that only its receiver finds
//! and reads, and a receiver finds its resources in the app data that the chain emits. [`EciesDiscovery`] is the
//! scheme of Anoma Galileo.

pub mod encryption;

use anoma_arm::digest::{AppData, Blob, DeletionCriterion};

pub use encryption::{Error, PublicKey, SecretKey};

/// The public keys of a receiver, in the encoding that an intent payload carries.
pub trait ReceiverAddress: Sized {
    fn to_bytes(&self) -> Vec<u8>;

    /// The address of bytes that `to_bytes` wrote, or `None` for other bytes.
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

/// A scheme that makes created resources discoverable for their receivers.
pub trait Discovery {
    /// What a creator needs to make a resource discoverable for a receiver.
    type Address: ReceiverAddress;
    /// What a receiver needs to find and read its resources.
    type SecretKeys;
    type Error: std::error::Error + Send + Sync + 'static;

    fn address(&self, secret_keys: &Self::SecretKeys) -> Self::Address;

    /// The app data of a created resource that only the receiver of `address` finds, with `plaintext` for it to read.
    fn app_data(&self, address: &Self::Address, plaintext: &[u8]) -> Result<AppData, Self::Error>;

    /// The plaintexts in the app data of a created resource, if the app data is for the secret keys.
    fn discover(&self, app_data: &AppData, secret_keys: &Self::SecretKeys) -> Option<Vec<Vec<u8>>>;
}

/// The plaintext of a discovery payload; only the receiver's ability to decrypt it counts.
pub const DISCOVERY_MESSAGE: [u8; 1] = [0];

/// The discovery of Anoma Galileo, with ECIES on secp256k1. The app data has two blobs: a discovery payload, which only
/// the discovery key of the receiver decrypts, and the plaintext, encrypted to its encryption key. A receiver finds
/// its resources by trial decryption of all discovery payloads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EciesDiscovery;

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

impl Discovery for EciesDiscovery {
    type Address = Receiver;
    type SecretKeys = ReceiverKeys;
    type Error = Error;

    fn address(&self, secret_keys: &ReceiverKeys) -> Receiver {
        secret_keys.receiver()
    }

    fn app_data(&self, receiver: &Receiver, plaintext: &[u8]) -> Result<AppData, Error> {
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

    fn discover(&self, app_data: &AppData, keys: &ReceiverKeys) -> Option<Vec<Vec<u8>>> {
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

impl ReceiverAddress for Receiver {
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
