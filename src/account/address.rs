use std::convert::TryFrom;
use std::fmt;

use base58::{FromBase58, ToBase58};
use bincode::{Decode, Encode};
use sha3::{Digest, Sha3_256, digest::FixedOutput};

use super::account::PublicKey;
use super::error::AddressParseError;

#[derive(PartialEq, Clone, Copy, Debug, Encode, Decode, Hash, Eq)]
pub struct Address([u8; 32]);

impl From<[u8; 32]> for Address {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for Address {
    type Error = AddressParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes = value.from_base58().map_err(AddressParseError::Base58)?;
        let array: [u8; 32] = bytes
            .try_into()
            .map_err(|_| AddressParseError::InputLength)?;
        Ok(Self(array))
    }
}

impl From<PublicKey> for Address {
    fn from(value: PublicKey) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(value);
        Self(hasher.finalize_fixed().into())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_base58())
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
