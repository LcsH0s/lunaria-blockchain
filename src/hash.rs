use bincode::{Decode, Encode};
use sha3::digest::generic_array::GenericArray;
use sha3::{Digest, Sha3_256, digest::FixedOutput};
use std::fmt;
use typenum::U32;

#[derive(PartialEq, Clone, Copy, Debug, Encode, Decode)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new<D: AsRef<[u8]>>(
        index: u64,
        timestamp: u128,
        previous_hash: Hash,
        data: D,
        nonce: u64,
    ) -> Self {
        let mut hasher = Sha3_256::new();

        hasher.update(index.to_ne_bytes());
        hasher.update(timestamp.to_ne_bytes());
        hasher.update(previous_hash);
        hasher.update(data);
        hasher.update(nonce.to_ne_bytes());

        hasher.finalize_fixed().into()
    }

    pub fn difficulty(&self) -> usize {
        let mut count = 0;
        for byte in self.0.iter() {
            if *byte == 0 {
                count += 8;
            } else {
                count += byte.leading_zeros() as usize;
                break;
            }
        }
        count
    }
}

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

#[derive(Clone)]
pub struct BlockHasher {
    state: Sha3_256,
}

impl BlockHasher {
    pub fn new<D: AsRef<[u8]>>(index: u64, timestamp: u128, previous_hash: Hash, data: D) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(index.to_be_bytes());
        hasher.update(timestamp.to_be_bytes());
        hasher.update(previous_hash.0);
        hasher.update(data.as_ref());
        BlockHasher { state: hasher }
    }

    pub fn hash_nonce(&mut self, nonce: u64) -> Hash {
        self.state.update(nonce.to_be_bytes());
        let bytes = self.state.finalize_reset();
        Hash(bytes.into())
    }
}
