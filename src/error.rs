use thiserror::Error;

use crate::block::{Block, BlockError};

#[derive(Error, Debug)]
pub enum ChainError {
    #[error("BlockError: {0}")]
    BlockError(#[from] BlockError),
    #[error("GenesisBlockError: {0}")]
    GenesisBlockError(Block),
    #[error("BlockNotFoundError: index:{0}")]
    BlockNotFound(u64),

    #[error("EncodeError: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),
    #[error("DecodeError: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),
}
