use anyhow::Result;
use ed25519_dalek::{PublicKey, SecretKey, Keypair};
use rand::thread_rng;

pub fn new_secret_key_hex() -> String {
    let secret_key = SecretKey::generate(&mut thread_rng());
    let bytes = secret_key.as_bytes();
    let hex = hex::encode(bytes);
    hex
}

pub fn key_pair_from_secret_key_hex(hex: &str) -> Result<Keypair> {
    let bytes = hex::decode(hex)?;
    let secret_key = SecretKey::from_bytes(&bytes)?;
    let public_key = PublicKey::from(&secret_key);
    let key_pair = Keypair {
        secret: secret_key,
        public: public_key,
    };
    Ok(key_pair)
}
