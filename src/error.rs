use thiserror::Error;

use crate::{block::Block, hash::Hash};

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

#[derive(Error, Debug)]
pub enum BlockError {
    #[error("InvalidHash: got: {got}, want: {want}")]
    InvalidHash { got: Hash, want: Hash },
    #[error("InvalidPreviousHash: got: {got}, want: {want}")]
    InvalidPreviousHash { got: Hash, want: Hash },
    #[error("InvalidIndex: got: {got}, want: {want}")]
    InvalidIndex { got: u64, want: u64 },

    #[error("EncodeError: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),
    #[error("DecodeError: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),
}
