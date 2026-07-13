use sha2::Digest;
use crate::blockchain::core::BlockChain;

impl BlockChain {
    pub fn proof_of_work(&self, last_proof: i64) -> i64 {
        let mut proof = 0;
        while self.valid_proof(last_proof, proof) == false {
            proof += 1;
        }

        proof
    }

    pub fn valid_proof(&self, last_proof: i64, proof: i64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = hex::encode(sha2::Sha256::digest(guess.as_bytes()));

        if guess_hash.ends_with("0000") {
            println!("New block!: {}", guess_hash);
        }

        guess_hash.ends_with("0000")
    }
}