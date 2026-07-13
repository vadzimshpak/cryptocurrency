use serde::{Serialize};
use sha2::{Digest, Sha256};

use crate::blockchain::transaction::Transaction;

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: f64,
    pub transactions: Vec<Transaction>,
    pub proof: i64,
    pub previous_hash: String,
}

impl Block {
    pub fn hash(&self) -> String {
        let block_string = serde_json::to_string(&self).unwrap();
        let hash = Sha256::digest(block_string.as_bytes());
        hex::encode(hash)
    }
}