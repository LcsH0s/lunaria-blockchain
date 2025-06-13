use bincode::{Decode, Encode};
use sha3::digest::generic_array::GenericArray;
use std::fmt;
use typenum::U32;

#[derive(PartialEq, Clone, Copy, Debug, Encode, Decode)]
pub struct Hash([u8; 32]);

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl From<GenericArray<u8, U32>> for Hash {
    fn from(array: GenericArray<u8, U32>) -> Self {
        Hash(array.into())
    }
}

impl From<[u8; 32]> for Hash {
    fn from(array: [u8; 32]) -> Self {
        Hash(array)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
