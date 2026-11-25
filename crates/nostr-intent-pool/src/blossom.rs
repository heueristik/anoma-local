//! The Blossom client of one key: the key signs the authorization of each request.

use blossom_rs::{BlossomClient, BlossomSigner};
use nostr::key::Keys;

pub(crate) fn client(server: &str, keys: Keys) -> BlossomClient {
    BlossomClient::new(vec![server.to_string()], Signer(keys))
}

struct Signer(Keys);

impl BlossomSigner for Signer {
    fn public_key_hex(&self) -> String {
        self.0.public_key().to_hex()
    }

    fn sign_schnorr(&self, message: &[u8; 32]) -> String {
        self.0.sign_schnorr(message).to_string()
    }
}
