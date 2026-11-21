//! The discovery of resources: the creator of a resource adds payloads to its app data that only its receiver finds
//! and reads, and a receiver finds its resources in the app data that the chain emits. [`GrumpkinDiscovery`] encrypts
//! field elements with Grumpkin and Poseidon2.

pub mod encryption;
pub mod grumpkin;

use anoma_arm::Fr;
use anoma_arm::app_data::AppData;

pub use grumpkin::{GrumpkinAddress, GrumpkinDiscovery, GrumpkinSecretKeys};

/// The public keys of a receiver, as the field elements that an intent payload carries.
pub trait ReceiverAddress: Sized {
    fn to_fields(&self) -> Vec<Fr>;

    /// The address of field elements that `to_fields` wrote, or `None` for others.
    fn from_fields(fields: &[Fr]) -> Option<Self>;
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
    fn app_data(&self, address: &Self::Address, plaintext: &[Fr]) -> Result<AppData, Self::Error>;

    /// The plaintexts in the app data of a created resource, if the app data is for the secret keys.
    fn discover(&self, app_data: &AppData, secret_keys: &Self::SecretKeys) -> Option<Vec<Vec<Fr>>>;
}
