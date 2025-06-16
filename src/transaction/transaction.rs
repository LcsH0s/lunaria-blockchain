use core::fmt;

use crate::account;
use bincode::{Decode, Encode};
use pqcrypto::{
    sign::falcon512::{self, SecretKey, verify_detached_signature},
    traits::sign::{DetachedSignature, PublicKey, SignedMessage},
};

use super::TransactionError;

pub type Signature = [u8; 752];

#[derive(PartialEq, Clone, Copy, Debug, Encode, Decode)]
#[repr(u8)]
pub enum TransactionType {
    Mint = 0,
    Transfer = 1,
}

impl TransactionType {
    pub fn to_bytes(self) -> [u8; 1] {
        [self as u8]
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Encode, Decode)]
pub struct Transaction {
    pub tx_type: TransactionType,
    pub from_address: account::Address,
    pub from_public_key: account::PublicKey,
    pub signature: Signature,
    pub to_address: account::Address,
    pub amount: u64,
}

pub fn sign(
    tx_type: TransactionType,
    from_address: account::Address,
    from_public_key: account::PublicKey,
    to_address: account::Address,
    amount: u64,
    secret_key: &SecretKey,
) -> Transaction {
    let mut msg: Vec<u8> = Vec::new();

    msg.extend(tx_type.to_bytes());
    msg.extend(from_address.as_ref());
    msg.extend(from_address.as_ref());
    msg.extend(from_public_key);
    msg.extend(to_address.as_ref());
    msg.extend(amount.to_le_bytes());

    let sig = pqcrypto::sign::falcon512::sign(&msg, secret_key);

    Transaction {
        tx_type,
        signature: sig
            .as_bytes()
            .try_into()
            .expect("Transaction signature was not 752 bytes"),
        from_address,
        from_public_key,
        to_address,
        amount,
    }
}

pub fn verify_signature(t: &Transaction) -> Result<(), TransactionError> {
    let mut msg: Vec<u8> = Vec::new();

    msg.extend(t.from_address.as_ref());
    msg.extend(&t.from_public_key);
    msg.extend(t.to_address.as_ref());
    msg.extend(t.amount.to_le_bytes());

    let sig = falcon512::DetachedSignature::from_bytes(&t.signature)?;
    let pk = falcon512::PublicKey::from_bytes(&t.from_public_key)?;

    if let Err(e) = verify_detached_signature(&sig, &msg, &pk) {
        return Err(TransactionError::VerificationError {
            transaction: t.clone(),
            source: e,
        });
    }

    Ok(())
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction:\n\
            \tType                : {:?}\n\
            \tFrom address        : {}\n\
            \tTo address          : {}\n\
            \tAmount              : {}",
            self.tx_type,
            self.from_address,
            // hex::encode(self.from_public_key),
            // hex::encode(self.signature),
            self.to_address,
            self.amount,
        )
    }
}
