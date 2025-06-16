use bincode::{Decode, Encode};
use pqcrypto::sign::falcon512::keypair;
use pqcrypto::traits::sign::{PublicKey, SecretKey};
use std::fs;

use crate::account::{self, Address};

use super::error::ClientError;

pub const DEFAULT_CREDS_LOCATION: &str = "./lunaria_wallet.bin";

#[derive(Encode, Decode)]
pub struct Client {
    pk: account::PublicKey,
    sk: account::SecretKey,
    address: Address,
}

impl Client {
    pub fn new() -> Self {
        let (pk, sk) = keypair();
        let pk: account::PublicKey = pk.as_bytes().try_into().expect("PublicKey bad length");
        let sk: account::SecretKey = sk.as_bytes().try_into().expect("PublicKey bad length");

        Self {
            pk,
            sk,
            address: Address::from(pk),
        }
    }

    pub fn from_default_path() -> Result<Self, ClientError> {
        let encoded = fs::read(DEFAULT_CREDS_LOCATION)?;
        let (client, _): (Self, usize) =
            bincode::decode_from_slice(&encoded, bincode::config::standard())?;
        Ok(client)
    }

    pub fn keypair(&self) -> (account::PublicKey, account::SecretKey) {
        (self.pk, self.sk)
    }

    pub fn address(&self) -> account::Address {
        self.address
    }

    pub fn save(&self) -> Result<(), ClientError> {
        let encoded = bincode::encode_to_vec(&self, bincode::config::standard())?;
        fs::write(DEFAULT_CREDS_LOCATION, encoded).map_err(ClientError::IOError)
    }
}
