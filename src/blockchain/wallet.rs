use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
    pub balance: f64,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;

        let private_key = SigningKey::generate(&mut csprng);
        let public_key = private_key.verifying_key();

        Self {
            public_key: bs58::encode(public_key.to_bytes()).into_string(),
            private_key: bs58::encode(private_key.to_bytes()).into_string(),
            balance: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_creation_works() {
        let wallet = Wallet::new();

        println!("Wallet: {:?}", wallet);
    }
}