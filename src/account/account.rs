use bincode::{Decode, Encode};

use super::address::Address;

pub type PublicKey = [u8; 897];
pub type SecretKey = [u8; 1281];

#[derive(PartialEq, Clone, Copy, Debug, Encode, Decode)]
pub struct Account {
    address: Address,
    pkey: Option<PublicKey>,
    balance: u64,
}
