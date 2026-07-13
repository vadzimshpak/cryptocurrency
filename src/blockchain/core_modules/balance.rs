use crate::blockchain::core::BlockChain;

impl BlockChain {
    pub fn balance(&self, address: String) -> u64 {
        let mut result = 0;
        
        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.recipient == address {
                    result += transaction.amount
                } else if transaction.sender == address { 
                    result -= transaction.amount
                }
            }
        }
        
        result
    }
}