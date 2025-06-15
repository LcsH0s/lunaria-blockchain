use crate::account::{Address, PublicKey};
use pqcrypto::sign::falcon512::{SecretKey, sign};
use pqcrypto::traits::sign::SignedMessage;

pub type Signature = [u8; 752];

pub struct Transaction {
    from_address: Address,
    from_public_key: PublicKey,
    signature: Signature,
    to_address: Address,
    amount: u64,
}

pub struct UnsignedTransaction {
    from_address: Address,
    from_public_key: PublicKey,
    to_address: Address,
    amount: u64,
}

impl UnsignedTransaction {
    pub fn new(
        from_address: Address,
        from_public_key: PublicKey,
        to_address: Address,
        amount: u64,
    ) -> Self {
        Self {
            from_address,
            from_public_key,
            to_address,
            amount,
        }
    }

    pub fn sign(self, sk: &SecretKey) -> Transaction {
        let mut msg: Vec<u8> = Vec::new();

        msg.extend(self.from_address.as_ref());
        msg.extend(self.from_public_key);
        msg.extend(self.to_address.as_ref());
        msg.extend(self.amount.to_le_bytes());

        let sig = sign(&msg, sk);

        Transaction {
            signature: sig
                .as_bytes()
                .try_into()
                .expect("Transaction signature was not 752 bytes"),
            from_address: self.from_address,
            from_public_key: self.from_public_key,
            to_address: self.to_address,
            amount: self.amount,
        }
    }
}
