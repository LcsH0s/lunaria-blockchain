// use lunaria::{block::Block, chain::Chain, hash::Hash};
// use std::fs;
use base58::ToBase58;
use pqcrypto::sign::falcon512::*;
use pqcrypto::traits::sign::PublicKey;
use sha3::{Digest, Sha3_256, digest::FixedOutput};

// const TEST_CHAIN_FILE: &str = "chain.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let encoded_chain: Vec<u8> = fs::read(TEST_CHAIN_FILE)?;
    // let mut chain = Chain::from_bytes(encoded_chain)?;

    // chain.verify()?;
    // println!("Chain:\n{chain}");

    // // let new_block = Block::new(2, 1, Hash::from([0u8; 32]), Vec::new());
    // // chain.append(new_block)?;

    // // fs::write(TEST_CHAIN_FILE, chain.encode()?)?;

    // let new_block = chain.forge(Vec::from([3u8; 32]))?;
    // println!("{new_block}");
    //
    let (pk, sk) = keypair();

    let pk_bytes = pk.as_bytes();

    // Hash the public key
    let mut hasher = Sha3_256::new();
    hasher.update(pk_bytes);
    let hash: [u8; 32] = hasher.finalize_fixed().into();

    // Convert to Base58 (compact and user-friendly)
    println!("Short Address: {}", &hash.to_base58());

    Ok(())
}
