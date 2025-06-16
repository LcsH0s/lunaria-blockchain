use thiserror::Error;

use crate::block::{Block, BlockError};
use crate::transaction::{Transaction, TransactionError};

#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("BlockError: {0}")]
    BlockError(#[from] BlockError),
    #[error("GenesisBlockError: {0}")]
    GenesisBlockError(Block),
    #[error("BlockNotFoundError: index:{0}")]
    BlockNotFound(u64),

    #[error("TransactionError: {0}")]
    TransactionError(#[from] TransactionError),
    #[error("ForbiddenMintTransaction: mint transaction outside of genesis block: {0:?}")]
    ForbiddenMintTransaction(Transaction),

    #[error("EncodeError: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),
    #[error("DecodeError: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),
}
