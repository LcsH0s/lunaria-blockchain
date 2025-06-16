use pqcrypto::traits::Error as SignatureError;
use pqcrypto::traits::sign::VerificationError;
use thiserror::Error;

use crate::account::Address;

use super::Transaction;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error(
        "VerificationError: failed to verify signature for transaction {transaction:?} : {source}"
    )]
    VerificationError {
        transaction: Transaction,
        #[source]
        source: VerificationError,
    },
    #[error("SignatureBadLength: {0}")]
    SignatureBadLength(#[from] SignatureError),

    #[error(
        "InsufficientBalance: address {address:?} has insufficient funds for transaction: {transaction:?}"
    )]
    InsufficientBalance {
        address: Address,
        transaction: Transaction,
    },
}
