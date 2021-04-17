use anyhow::{Result, bail};
use ed25519_dalek::{PublicKey, SecretKey, Keypair};
use rand::thread_rng;

use bech32::{FromBase32, ToBase32, Variant};

pub fn new_secret_key() -> Result<String> {
    let secret_key = SecretKey::generate(&mut thread_rng());
    let bytes = secret_key.as_bytes();
    let encoded = bech32::encode("gt", bytes.to_base32(), Variant::Bech32m)?;
    Ok(encoded)
}

pub fn key_pair_from_secret_key(key: &str) -> Result<Keypair> {
    let (hrp, data, variant) = bech32::decode(key)?;

    if hrp != "geonft" {
        bail!("wrong HRP in secret key decoding");
    }

    if variant != Variant::Bech32 {
        bail!("wrong bech32 variant in secret key decoding");
    }

    let bytes = Vec::<u8>::from_base32(&data)?;
    let secret_key = SecretKey::from_bytes(&bytes)?;
    let public_key = PublicKey::from(&secret_key);
    let key_pair = Keypair {
        secret: secret_key,
        public: public_key,
    };
    Ok(key_pair)
}
