use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;

use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut blockchain = BlockChain {
            chain: vec![],
            current_transactions: vec![],
        };

        blockchain.new_block(0, String::from("0"));
        blockchain.inner_transaction("core0000".to_string(), "0".to_string(), 1_000_000);

        blockchain
    }

    pub fn new_block(&mut self, proof: i64, previous_hash: String) -> &Block {
        self.chain.push(Block {
            index: self.chain.len() + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
            transactions: self.current_transactions.clone(),
            proof,
            previous_hash,
        });

        self.current_transactions.clear();

        self.last_block()
    }

    pub fn new_transaction(&mut self, sender: String, recipient: String, signature: String, amount: u64) -> bool {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        let transaction = Transaction { sender, recipient, amount, timestamp, signature };

        if !transaction.verify() {
            return false;
        }

        self.current_transactions.push(transaction);

        true
    }

    pub fn inner_transaction(&mut self, sender: String, recipient: String, amount: u64) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        let signature = "inner_sys".to_string();

        self.current_transactions.push(Transaction { sender, recipient, amount, timestamp, signature });
    }

    pub fn last_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::transaction::Transaction;

    fn create_test_block() -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        Block {
            index: 1,
            timestamp: 1234567890.0,
            transactions: vec![
                Transaction {
                    sender: String::from("Alice"),
                    recipient: String::from("Bob"),
                    amount: 10,
                    signature: String::from("Alice"),
                    timestamp,

                },
            ],
            proof: 100,
            previous_hash: String::from("previous_hash"),
        }
    }

    fn create_test2_block() -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        Block {
            index: 2,
            timestamp: 1234567890.0,
            transactions: vec![
                Transaction {
                    sender: String::from("Alice"),
                    recipient: String::from("Bob"),
                    amount: 10,
                    signature: String::from("Alice"),
                    timestamp
                },
            ],
            proof: 100,
            previous_hash: String::from("previous_hash"),
        }
    }

    #[test]
    fn hash_returns_valid_sha256_hex_string() {
        let hash = create_test_block().hash();

        println!("Hash: {}", hash);
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hash_is_deterministic_for_same_block() {
        let first_hash = create_test_block().hash();
        let second_hash = create_test_block().hash();

        assert_eq!(first_hash, second_hash);
    }

    #[test]
    fn hash_changes_when_block_changes() {
        let first_hash = create_test_block().hash();
        let second_hash = create_test2_block().hash();

        assert_ne!(first_hash, second_hash);
    }
}