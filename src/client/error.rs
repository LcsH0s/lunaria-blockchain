use bincode::error::{DecodeError, EncodeError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("EncodeError: {0}")]
    EncodeError(#[from] EncodeError),
    #[error("DecodeError: {0}")]
    DecodeError(#[from] DecodeError),
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
}
