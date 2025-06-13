use crate::error::BlockError;
use crate::hash::Hash;
use bincode::{Decode, Encode, config};
use sha3::{Digest, Sha3_256, digest::FixedOutput};
use std::fmt;

pub const GENESIS_INDEX: u64 = 0;

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct Block {
    index: u64,
    timestamp: u128,
    hash: Hash,
    previous_hash: Hash,
    data: Vec<u8>,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, previous_hash: Hash, data: Vec<u8>) -> Self {
        let mut hasher = Sha3_256::new();

        hasher.update(index.to_ne_bytes());
        hasher.update(timestamp.to_ne_bytes());
        hasher.update(previous_hash);
        hasher.update(&data);

        let hash: Hash = hasher.finalize_fixed().into();

        Block {
            index,
            timestamp,
            hash,
            previous_hash,
            data,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, BlockError> {
        let config = config::standard();
        let (decoded, _): (Self, usize) = bincode::decode_from_slice(&bytes, config)?;
        Ok(decoded)
    }

    pub fn genesis() -> Self {
        Self::new(0, 0, Hash::from([0u8; 32]), Vec::new())
    }

    pub fn verify(&self, prev: &Block) -> Result<(), BlockError> {
        if self.index() != prev.index() + 1 {
            return Err(BlockError::InvalidIndex {
                got: self.index(),
                want: prev.index() + 1,
            });
        }

        if self.previous_hash != *prev.hash() {
            return Err(BlockError::InvalidPreviousHash {
                got: self.previous_hash.clone(),
                want: prev.hash().clone(),
            });
        }

        self.verify_hash()
    }

    pub fn verify_hash(&self) -> Result<(), BlockError> {
        let mut hasher = Sha3_256::new();

        hasher.update(self.index.to_ne_bytes());
        hasher.update(self.timestamp.to_ne_bytes());
        hasher.update(&self.previous_hash);
        hasher.update(&self.data);

        let computed: Hash = hasher.finalize_fixed().into();
        let current = self.hash();

        if *current != computed {
            Err(BlockError::InvalidHash {
                got: self.hash().clone(),
                want: computed,
            })
        } else {
            Ok(())
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, BlockError> {
        let config = config::standard();
        bincode::encode_to_vec(&self, config).map_err(BlockError::from)
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn index(&self) -> u64 {
        self.index
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block #{}\n\
             Timestamp     : {}\n\
             Hash          : {}\n\
             Previous Hash : {}\n\
             Data          : {} bytes\n\
             Data (hex)    : {}",
            self.index,
            self.timestamp,
            self.hash,
            self.previous_hash,
            self.data.len(),
            hex::encode(&self.data)
        )
    }
}
