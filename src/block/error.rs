use thiserror::Error;

use super::hash::BlockHash;

#[derive(Error, Debug)]
pub enum BlockError {
    #[error("InvalidHash: got: {got}, want: {want}")]
    InvalidHash { got: BlockHash, want: BlockHash },
    #[error("InvalidPreviousHash: got: {got}, want: {want}")]
    InvalidPreviousHash { got: BlockHash, want: BlockHash },
    #[error("InvalidIndex: got: {got}, want: {want}")]
    InvalidIndex { got: u64, want: u64 },

    #[error("EncodeError: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),
    #[error("DecodeError: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),

    #[error("InvalidNonce: {0}")]
    InvalidNonce(u64),
    #[error("NonceTooHard")]
    NonceTooHard,
}
