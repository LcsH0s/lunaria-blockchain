use super::error::BlockError;
use super::hash::{BlockHash, BlockHasher};

use bincode::{Decode, Encode, config};
use rayon::prelude::*;
use sha3::{Digest, Sha3_256, digest::FixedOutput};

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

pub const DIFFICULTY: usize = 8;

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct Block {
    index: u64,
    timestamp: u128,
    hash: BlockHash,
    previous_hash: BlockHash,
    data: Vec<u8>,
    nonce: u64,
}

impl Block {
    pub fn forge<D: AsRef<[u8]> + Send + Sync>(
        index: u64,
        timestamp: u128,
        previous_hash: BlockHash,
        data: D,
    ) -> Result<Self, BlockError> {
        Self::forge_with_difficulty(index, timestamp, previous_hash, data, DIFFICULTY)
    }

    fn forge_with_difficulty<D: AsRef<[u8]> + Send + Sync>(
        index: u64,
        timestamp: u128,
        previous_hash: BlockHash,
        data: D,
        difficulty: usize,
    ) -> Result<Self, BlockError> {
        let base_hasher = BlockHasher::new(index, timestamp, previous_hash, data.as_ref());
        let found = AtomicBool::new(false);

        let max_attempts = 1_000_000_000u64;

        let result = (0..max_attempts).into_par_iter().find_map_first(|nonce| {
            if found.load(Ordering::Acquire) {
                return None;
            }

            let mut hasher = base_hasher.clone();
            let hash = hasher.hash_nonce(nonce);

            if hash.difficulty() >= difficulty {
                found.store(true, Ordering::Relaxed);
                Some((nonce, hash))
            } else {
                None
            }
        });

        let (nonce, hash) = result.ok_or(BlockError::NonceTooHard)?;

        Ok(Block {
            index,
            timestamp,
            previous_hash,
            data: data.as_ref().to_vec(),
            nonce,
            hash,
        })
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, BlockError> {
        let config = config::standard();
        let (decoded, _): (Self, usize) = bincode::decode_from_slice(&bytes, config)?;
        Ok(decoded)
    }

    pub fn genesis() -> Result<Self, BlockError> {
        Self::forge_with_difficulty(0, 0, BlockHash::from([0u8; 32]), Vec::new(), DIFFICULTY)
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

        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(&self.previous_hash);
        hasher.update(&self.data);

        let computed: BlockHash = hasher.finalize_fixed().into();
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

    pub fn hash(&self) -> &BlockHash {
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
             Data (hex)    : {}\n\
             Nonce         : {}",
            self.index,
            self.timestamp,
            self.hash,
            self.previous_hash,
            self.data.len(),
            hex::encode(&self.data),
            self.nonce
        )
    }
}
