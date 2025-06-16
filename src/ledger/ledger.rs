use crate::account::Address;
use crate::block::Block;
use crate::transaction::{self, Transaction, TransactionError, TransactionType};

use super::error::LedgerError;

use bincode::{Decode, Encode, config};
use std::collections::HashMap;
use std::time::SystemTime;

pub const TRANSACTION_COST: u64 = 0;

#[derive(Debug, Clone, Encode, Decode)]
pub struct Ledger {
    chain: Vec<Block>,
    state: HashMap<Address, u64>,
}

impl Ledger {
    pub fn new() -> Result<Self, LedgerError> {
        let mut ledger = Ledger {
            chain: Vec::new(),
            state: HashMap::new(),
        };

        ledger.genesis()?;

        Ok(ledger)
    }

    fn genesis(&mut self) -> Result<(), LedgerError> {
        let genesis = Block::genesis()?;

        for t in genesis.transactions() {
            self.apply_transaction_unchecked(t)?;
        }

        self.chain.push(genesis);

        Ok(())
    }

    pub fn state(&self) -> HashMap<Address, u64> {
        self.state.clone()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, LedgerError> {
        let config = config::standard();
        let (decoded, _): (Self, usize) = bincode::decode_from_slice(&bytes, config)?;
        // TODO: Add entire ledger re-validation from the start
        Ok(decoded)
    }

    pub fn forge(&self, transactions: Vec<Transaction>) -> Result<Block, LedgerError> {
        let last_block = self.last()?;
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Block::forge(
            last_block.index() + 1,
            timestamp,
            last_block.hash().clone(),
            transactions,
        )
        .map_err(LedgerError::from)
    }

    pub fn apply_transactions(&mut self, block: &Block) -> Result<(), LedgerError> {
        for t in block.transactions() {
            if t.tx_type == TransactionType::Mint && block.index() != 0 {
                return Err(LedgerError::ForbiddenMintTransaction(t.clone()));
            }

            if let Err(e) = transaction::verify_signature(&t) {
                return Err(e.into());
            }

            // TODO: for now, entire block is refused if at least one transaction is invalid
            match self.dry_run_transaction(t) {
                Err(e) => return Err(LedgerError::TransactionError(e)),
                Ok(_) => {
                    self.apply_transaction_unchecked(t)?;
                }
            }
        }

        Ok(())
    }

    fn dry_run_transaction(&self, t: &Transaction) -> Result<(), TransactionError> {
        match self.state.get(&t.from_address) {
            Some(balance) => {
                if *balance < t.amount + TRANSACTION_COST {
                    return Err(TransactionError::InsufficientBalance {
                        address: t.from_address,
                        transaction: t.clone(),
                    });
                } else {
                    return Ok(());
                }
            }
            None => Err(TransactionError::InsufficientBalance {
                address: t.from_address,
                transaction: t.clone(),
            }),
        }
    }

    fn apply_transaction_unchecked(&mut self, t: &Transaction) -> Result<(), LedgerError> {
        if t.tx_type != TransactionType::Mint {
            let from_balance =
                self.state
                    .get(&t.from_address)
                    .ok_or(LedgerError::TransactionError(
                        TransactionError::InsufficientBalance {
                            address: t.from_address,
                            transaction: t.clone(),
                        },
                    ))?;

            self.state.insert(
                t.from_address,
                *from_balance - (t.amount + TRANSACTION_COST),
            );
        }

        match self.state.get(&t.to_address) {
            Some(to_balance) => {
                self.state.insert(t.to_address, to_balance + t.amount);
            }
            None => {
                self.state.insert(t.to_address, t.amount);
            }
        }

        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, LedgerError> {
        let config = config::standard();
        bincode::encode_to_vec(&self, config).map_err(LedgerError::from)
    }

    pub fn last(&self) -> Result<&Block, LedgerError> {
        self.chain.last().ok_or(LedgerError::BlockNotFound(0))
    }
}

impl std::fmt::Display for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Blockchain - Total Blocks: {}", self.chain.len())?;
        for (i, block) in self.chain.iter().enumerate() {
            writeln!(f, "\n=== Block {} ===\n{}", i, block)?;
        }
        Ok(())
    }
}
