use lunaria::{block::Block, chain::Chain, hash::Hash};
use std::fs;

const TEST_CHAIN_FILE: &str = "chain.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let encoded_chain: Vec<u8> = fs::read(TEST_CHAIN_FILE)?;
    let mut chain = Chain::from_bytes(encoded_chain)?;

    chain.verify()?;
    println!("Chain:\n{chain}");

    // let new_block = Block::new(2, 1, Hash::from([0u8; 32]), Vec::new());
    // chain.append(new_block)?;

    // fs::write(TEST_CHAIN_FILE, chain.encode()?)?;

    let new_block = chain.forge(Vec::from([3u8; 32]))?;
    println!("{new_block}");

    Ok(())
}
