use arm::hash::{self, hash};

/// A P-256 public key: the big-endian bytes of its coordinates.
pub struct PublicKey {
    pub x: [u8; 32],
    pub y: [u8; 32],
}

impl PublicKey {
    /// The hash of the key, as the label ref of a token names its issuer and the value ref its owner.
    pub fn hash(self) -> Field {
        let (x_high, x_low) = halves(self.x);
        let (y_high, y_low) = halves(self.y);
        hash(hash::PUBLIC_KEY, [x_high, x_low, y_high, y_low])
    }

    /// Whether `signature` is an ECDSA signature of the key, with a low s, over the SHA-256 hash of the 32 big-endian
    /// bytes of `message`, as keriox signs data.
    pub fn verify(self, signature: [u8; 64], message: Field) -> bool {
        let digest = sha256::digest(message.to_be_bytes::<32>());
        std::ecdsa_secp256r1::verify_signature(self.x, self.y, signature, digest)
    }
}

/// The high and the low 16 of 32 big-endian bytes, each as a field element.
fn halves(bytes: [u8; 32]) -> (Field, Field) {
    let mut high = 0;
    let mut low = 0;
    for i in 0..16 {
        high = high * 256 + bytes[i] as Field;
        low = low * 256 + bytes[i + 16] as Field;
    }
    (high, low)
}

#[test]
fn halves_reads_big_endian_bytes() {
    let mut bytes = [0; 32];
    bytes[15] = 1;
    bytes[30] = 2;
    assert_eq(halves(bytes), (1, 512));
}
