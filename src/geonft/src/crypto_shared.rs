use anyhow::{bail, Result};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};

use base64;
use bech32::{FromBase32, ToBase32, Variant};
use sha256::digest_bytes;

pub static ACCOUNT_SECRET_KEY_HRP: &'static str = "gas";
pub static ACCOUNT_PUBLIC_KEY_HRP: &'static str = "gap";
pub static TREASURE_SECRET_KEY_HRP: &'static str = "gs";
pub static TREASURE_PUBLIC_KEY_HRP: &'static str = "gp";
pub static TREASURE_SECRET_URL_PREFIX: &'static str = "http://localhost:8000/claim?key=";

pub fn keypair_from_account_secret_key(key: &str) -> Result<Keypair> {
    let secret_key = decode_account_secret_key(key)?;
    let public_key = PublicKey::from(&secret_key);
    let keypair = Keypair {
        secret: secret_key,
        public: public_key,
    };
    Ok(keypair)
}

pub fn keypair_from_treasure_secret_key(key: &str) -> Result<Keypair> {
    let secret_key = decode_treasure_secret_key(key)?;
    let public_key = PublicKey::from(&secret_key);
    let keypair = Keypair {
        secret: secret_key,
        public: public_key,
    };
    Ok(keypair)
}

pub fn keypair_to_treasure_secret_url(keypair: &Keypair) -> Result<String> {
    let secret_key_string = encode_treasure_secret_key(&keypair.secret)?;
    let url = format!("{}{}", TREASURE_SECRET_URL_PREFIX, secret_key_string);
    Ok(url)
}

pub fn treasure_secret_url_to_keypair(url: &str) -> Result<Keypair> {
    if !url.starts_with(TREASURE_SECRET_URL_PREFIX) {
        bail!("incorrect URL prefix for secret key");
    }

    let key = url.split_at(TREASURE_SECRET_URL_PREFIX.len()).1;
    keypair_from_treasure_secret_key(key)
}

pub fn encode_account_secret_key(key: &SecretKey) -> Result<String> {
    encode_secret_key(key, ACCOUNT_SECRET_KEY_HRP)
}

pub fn encode_treasure_secret_key(key: &SecretKey) -> Result<String> {
    encode_secret_key(key, TREASURE_SECRET_KEY_HRP)
}

fn encode_secret_key(key: &SecretKey, hrp: &str) -> Result<String> {
    let bytes = key.as_bytes();
    let encoded = bech32::encode(hrp, bytes.to_base32(), Variant::Bech32m).e()?;
    Ok(encoded)
}

pub fn decode_account_secret_key(key: &str) -> Result<SecretKey> {
    decode_secret_key(key, ACCOUNT_SECRET_KEY_HRP)
}

pub fn decode_treasure_secret_key(key: &str) -> Result<SecretKey> {
    decode_secret_key(key, TREASURE_SECRET_KEY_HRP)
}

pub fn decode_secret_key(key: &str, hrp: &str) -> Result<SecretKey> {
    let (actual_hrp, data, variant) = bech32::decode(key).e()?;

    if actual_hrp != hrp {
        bail!("wrong HRP in secret key decoding");
    }

    if variant != Variant::Bech32m {
        bail!("wrong bech32 variant in secret key decoding");
    }

    let bytes = Vec::<u8>::from_base32(&data).e()?;
    let key = SecretKey::from_bytes(&bytes).e()?;
    Ok(key)
}

pub fn encode_account_public_key(key: &PublicKey) -> Result<String> {
    encode_public_key(key, ACCOUNT_PUBLIC_KEY_HRP)
}

pub fn encode_treasure_public_key(key: &PublicKey) -> Result<String> {
    encode_public_key(key, TREASURE_PUBLIC_KEY_HRP)
}

fn encode_public_key(key: &PublicKey, hrp: &str) -> Result<String> {
    let bytes = key.as_bytes();
    let encoded = bech32::encode(hrp, bytes.to_base32(), Variant::Bech32m).e()?;
    Ok(encoded)
}

pub fn decode_account_public_key(key: &str) -> Result<PublicKey> {
    decode_public_key(key, ACCOUNT_PUBLIC_KEY_HRP)
}

pub fn decode_treasure_public_key(key: &str) -> Result<PublicKey> {
    decode_public_key(key, TREASURE_PUBLIC_KEY_HRP)
}

fn decode_public_key(key: &str, hrp: &str) -> Result<PublicKey> {
    let (actual_hrp, data, variant) = bech32::decode(key).e()?;

    if actual_hrp != hrp {
        bail!("wrong HRP in public key decoding");
    }

    if variant != Variant::Bech32m {
        bail!("wrong bech32 variant in public key decoding");
    }

    let bytes = Vec::<u8>::from_base32(&data).e()?;
    let key = PublicKey::from_bytes(&bytes).e()?;
    Ok(key)
}

/// With the account secret key, sign
///
/// - "plant", appended with
/// - the treasure public key
pub fn sign_plant_request_for_account(
    account_secret_key: SecretKey,
    treasure_public_key: PublicKey,
) -> Result<Signature> {
    let mut message = Vec::from("plant");
    message.extend_from_slice(&treasure_public_key.to_bytes());

    create_signature(&message, &account_secret_key)
}

/// With the account public key, verify
/// the plant request signature.
pub fn verify_plant_request_for_account(
    account_public_key: PublicKey,
    treasure_public_key: PublicKey,
    signature: Signature,
) -> Result<()> {
    let mut message = Vec::from("plant");
    message.extend_from_slice(&treasure_public_key.to_bytes());

    verify_signature(&message, &signature, &account_public_key)
}

/// With the treasure secret key, sign
///
/// - "plant", appended with
/// - the account public key, appended with
/// - the hash of the treasure image
pub fn sign_plant_request_for_treasure(
    treasure_secret_key: SecretKey,
    account_public_key: PublicKey,
    treasure_hash: &[u8],
) -> Result<Signature> {
    let mut message = Vec::from("plant");
    message.extend_from_slice(&account_public_key.to_bytes());
    message.extend_from_slice(treasure_hash);

    create_signature(&message, &treasure_secret_key)
}

/// With the treasure public key, verify
/// the plant request signature.
pub fn verify_plant_request_for_treasure(
    treasure_public_key: PublicKey,
    account_public_key: PublicKey,
    treasure_hash: &[u8],
    signature: Signature,
) -> Result<()> {
    let mut message = Vec::from("plant");
    message.extend_from_slice(&account_public_key.to_bytes());
    message.extend_from_slice(treasure_hash);

    verify_signature(&message, &signature, &treasure_public_key)
}

/// With the account secret key, sign
///
/// - "claim", appended with
/// - the treasure public key
pub fn sign_claim_request_for_account(
    account_secret_key: SecretKey,
    treasure_public_key: PublicKey,
) -> Result<Signature> {
    let mut message = Vec::from("claim");
    message.extend_from_slice(&treasure_public_key.to_bytes());

    create_signature(&message, &account_secret_key)
}

/// With the account public key, verify
/// the claim request signature.
pub fn verify_claim_request_for_account(
    account_public_key: PublicKey,
    treasure_public_key: PublicKey,
    signature: Signature,
) -> Result<()> {
    let mut message = Vec::from("claim");
    message.extend_from_slice(&treasure_public_key.to_bytes());

    verify_signature(&message, &signature, &account_public_key)
}

/// With the treasure secret key, sign
///
/// - "claim", appended with
/// - the account public key
pub fn sign_claim_request_for_treasure(
    treasure_secret_key: SecretKey,
    account_public_key: PublicKey,
) -> Result<Signature> {
    let mut message = Vec::from("claim");
    message.extend_from_slice(&account_public_key.to_bytes());

    create_signature(&message, &treasure_secret_key)
}

/// With the treasure public key, verify
/// the claim request signature.
pub fn verify_claim_request_for_treasure(
    treasure_public_key: PublicKey,
    account_public_key: PublicKey,
    signature: Signature,
) -> Result<()> {
    let mut message = Vec::from("claim");
    message.extend_from_slice(&account_public_key.to_bytes());

    verify_signature(&message, &signature, &treasure_public_key)
}

pub fn encode_signature(sig: &Signature) -> Result<String> {
    let bytes = sig.to_bytes();
    let encoded = base64::encode(bytes);
    Ok(encoded)
}

// Decodes a base64 encoded signature
pub fn decode_signature(sig: &str) -> Result<Signature> {
    let decoded = base64::decode(sig.as_bytes()).e()?;
    let mut decoded_array = [0; 64];
    decoded_array.copy_from_slice(decoded.as_slice());

    let signature = Signature::new(decoded_array);
    Ok(signature)
}

pub fn create_signature(message: &[u8], secret_key: &SecretKey) -> Result<Signature> {
    let secret_key = SecretKey::from_bytes(&secret_key.to_bytes()).e()?;
    let public_key = PublicKey::from(&secret_key);
    let keypair = Keypair {
        secret: secret_key,
        public: public_key,
    };

    let signature = keypair.try_sign(message).e()?;
    Ok(signature)
}

pub fn verify_signature(
    message: &[u8],
    signature: &Signature,
    public_key: &PublicKey,
) -> Result<()> {
    Ok(public_key.verify_strict(message, signature).e()?)
}

pub fn get_hash(data: &str) -> Result<String> {
    Ok(digest_bytes(data.as_bytes()))
}

/*
pub fn decode_image(image: &str) -> Result<Vec<u8>> {
    Ok(base64::decode(image.as_bytes()).e()?)
}
*/

pub trait ResultWrapper<T> {
    fn e(self) -> Result<T>;
}

#[cfg(feature = "std-errors")]
mod err {
    use super::ResultWrapper;

    impl<T> ResultWrapper<T> for Result<T, bech32::Error> {
        fn e(self) -> Result<T, anyhow::Error> {
            Ok(self?)
        }
    }

    impl<T> ResultWrapper<T> for Result<T, ed25519_dalek::ed25519::Error> {
        fn e(self) -> Result<T, anyhow::Error> {
            Ok(self?)
        }
    }

    impl<T> ResultWrapper<T> for Result<T, base64::DecodeError> {
        fn e(self) -> Result<T, anyhow::Error> {
            Ok(self?)
        }
    }
}

#[cfg(not(feature = "std-errors"))]
mod err {
    use super::ResultWrapper;

    impl<T> ResultWrapper<T> for Result<T, bech32::Error> {
        fn e(self) -> Result<T, anyhow::Error> {
            self.map_err(|e| anyhow::anyhow!("{}", e))
        }
    }

    impl<T> ResultWrapper<T> for Result<T, ed25519_dalek::ed25519::Error> {
        fn e(self) -> Result<T, anyhow::Error> {
            self.map_err(|e| anyhow::anyhow!("{}", e))
        }
    }

    impl<T> ResultWrapper<T> for Result<T, base64::DecodeError> {
        fn e(self) -> Result<T, anyhow::Error> {
            self.map_err(|e| anyhow::anyhow!("{}", e))
        }
    }
}