use serde::{Serialize};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub timestamp: f64,
    pub signature: String,
}

impl Transaction {
    pub fn verify(&self) -> bool {
        println!("Verifying transaction: {}", &self.sender);
        let public_key_bytes = bs58::decode(&self.sender).into_vec().unwrap();
        let public_key_array: [u8; 32] = public_key_bytes
            .try_into()
            .map_err(|_| "Public key is not 32 bytes long")
            .expect("Public key is not 32 bytes long");

        let public_key = VerifyingKey::from_bytes(&public_key_array).unwrap();

        let signature_bytes = bs58::decode(&self.signature).into_vec().unwrap();
        let signature_array: [u8; 64] = signature_bytes
            .try_into()
            .map_err(|_| "Signature is not 64 bytes long")
            .expect("Signature is not 64 bytes long");

        let signature = Signature::from_bytes(&signature_array);

        let data = format!("{}:{}:{}", self.sender, self.recipient, self.amount);

        public_key.verify(&data.as_bytes(), &signature).is_ok()
    }

    pub fn sign(&self, private_key: String) -> String {
        let private_key_bytes = bs58::decode(&private_key).into_vec().unwrap();
        let private_key_array: [u8; 32] = private_key_bytes
            .try_into()
            .map_err(|_| "Private key is not 32 bytes long")
            .expect("Private key is not 32 bytes long");

        let private_key = SigningKey::from_bytes(&private_key_array);
        let data = format!("{}:{}:{}", self.sender, self.recipient, self.amount);


        bs58::encode(private_key.sign(&data.as_bytes()).to_bytes()).into_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::wallet::Wallet;
    use super::*;

    #[test]
    fn transaction_signature() {
        let wallet1 = Wallet::new();
        let wallet2 = Wallet::new();

        let mut transaction = Transaction {
            sender: wallet1.public_key.clone(),
            recipient: wallet2.public_key.clone(),
            amount: 1,
            timestamp: 1234567890.0,
            signature: "".to_string(),
        };

        transaction.signature = transaction.sign(wallet1.private_key.clone());

        println!("Signature: {}", transaction.signature);
        println!("Sender: {}", wallet1.public_key);
        println!("Recipient: {}", wallet2.public_key);

        assert!(transaction.verify());
    }
}