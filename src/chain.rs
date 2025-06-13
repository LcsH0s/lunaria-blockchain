use crate::{block::Block, error::ChainError};

use bincode::{Decode, Encode, config};
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct Chain {
    chain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Result<Self, ChainError> {
        let genesis = Block::genesis()?;

        Ok(Chain {
            chain: vec![genesis],
        })
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, ChainError> {
        let config = config::standard();
        let (decoded, _): (Self, usize) = bincode::decode_from_slice(&bytes, config)?;
        Ok(decoded)
    }

    pub fn verify(&self) -> Result<(), ChainError> {
        let genesis_block = self.chain.first().ok_or(ChainError::BlockNotFound(0))?;

        if genesis_block.index() != 0 {
            return Err(ChainError::GenesisBlockError(genesis_block.clone()));
        }

        if self.chain.len() == 1 {
            return Ok(());
        }

        for blocks in self.chain.windows(2) {
            if let Err(e) = blocks[1].verify(&blocks[0]) {
                return Err(ChainError::from(e));
            }
        }

        Ok(())
    }

    pub fn forge<D>(&self, data: D) -> Result<Block, ChainError>
    where
        D: AsRef<[u8]> + Send + Sync,
    {
        let last_block = self.last()?;
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Block::forge(
            last_block.index() + 1,
            timestamp,
            last_block.hash().clone(),
            data,
        )
        .map_err(ChainError::from)
    }

    pub fn append(&mut self, block: Block) -> Result<(), ChainError> {
        let last_block = self.last()?;

        if let Err(e) = block.verify(&last_block) {
            Err(ChainError::BlockError(e))
        } else {
            self.chain.push(block);
            Ok(())
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, ChainError> {
        let config = config::standard();
        bincode::encode_to_vec(&self, config).map_err(ChainError::from)
    }

    pub fn last(&self) -> Result<&Block, ChainError> {
        self.chain.last().ok_or(ChainError::BlockNotFound(0))
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Blockchain - Total Blocks: {}", self.chain.len())?;
        for (i, block) in self.chain.iter().enumerate() {
            writeln!(f, "\n=== Block {} ===\n{}", i, block)?;
        }
        Ok(())
    }
}
