use super::error::BlockError;
use super::hash::{BlockHash, BlockHasher};
use crate::account::Address;
use crate::transaction::{Transaction, TransactionType};

use std::sync::atomic::{AtomicBool, Ordering};
use std::{fmt, u64};

use bincode::{Decode, Encode, config};
use rayon::prelude::*;
use sha3::{Digest, Sha3_256, digest::FixedOutput};

pub const DIFFICULTY: usize = 8;

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct Block {
    index: u64,
    timestamp: u128,
    hash: BlockHash,
    previous_hash: BlockHash,
    transactions: Vec<Transaction>,
    nonce: u64,
}

impl Block {
    pub fn forge(
        index: u64,
        timestamp: u128,
        previous_hash: BlockHash,
        transactions: Vec<Transaction>,
    ) -> Result<Self, BlockError> {
        Self::forge_with_difficulty(index, timestamp, previous_hash, transactions, DIFFICULTY)
    }

    fn forge_with_difficulty(
        index: u64,
        timestamp: u128,
        previous_hash: BlockHash,
        transactions: Vec<Transaction>,
        difficulty: usize,
    ) -> Result<Self, BlockError> {
        let encoded_transactions =
            bincode::encode_to_vec(&transactions, bincode::config::standard())
                .map_err(BlockError::TransactionEncodeError)?;
        let base_hasher = BlockHasher::new(index, timestamp, previous_hash, encoded_transactions);
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
            transactions,
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
        let genesis_transactions = vec![Transaction {
            tx_type: TransactionType::Mint,
            from_address: Address::from([0u8; 32]),
            from_public_key: [0u8; 897],
            signature: [0u8; 752],
            to_address: Address::try_from("9JEuZSy4CmRM8wMiE368Bx5jkgK5SLH1KvRDiUcNRjsV")
                .map_err(BlockError::GenesisTransactionError)?,
            amount: u64::MAX / 2,
        }];

        Self::forge_with_difficulty(
            0,
            0,
            BlockHash::from([0u8; 32]),
            genesis_transactions,
            DIFFICULTY,
        )
    }

    pub fn verify_hash(&self) -> Result<(), BlockError> {
        let mut hasher = Sha3_256::new();

        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(&self.previous_hash);

        let encoded_transactions =
            bincode::encode_to_vec(&self.transactions, bincode::config::standard())
                .map_err(BlockError::TransactionEncodeError)?;

        hasher.update(&encoded_transactions);

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

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let transactions_str = self
            .transactions
            .iter()
            .map(|tx| format!("{}", tx))
            .collect::<Vec<_>>()
            .join("\n    ");

        write!(
            f,
            "Block #{}\n\
             Timestamp      : {}\n\
             Hash           : {}\n\
             Previous Hash  : {}\n\
             Transactions   : [\n    {}\n]\n\
             Nonce          : {}",
            self.index, self.timestamp, self.hash, self.previous_hash, transactions_str, self.nonce
        )
    }
}
