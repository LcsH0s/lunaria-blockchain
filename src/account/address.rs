use bincode::{Decode, Encode};
use sha3::{Digest, Sha3_256, digest::FixedOutput};

use super::account::PublicKey;

#[derive(PartialEq, Clone, Copy, Debug, Encode, Decode)]
pub struct Address([u8; 32]);

impl From<[u8; 32]> for Address {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<PublicKey> for Address {
    fn from(value: PublicKey) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(value);
        Self(hasher.finalize_fixed().into())
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
