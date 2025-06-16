use thiserror::Error;

use base58::FromBase58Error;

#[derive(Error, Debug)]
pub enum AddressParseError {
    #[error("Base58 decoding error: {0:?}")]
    Base58(FromBase58Error),
    #[error("InputLength: Invalid address length (expected 32 bytes)")]
    InputLength,
}
