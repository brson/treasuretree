use anyhow::{Result, bail};
use ed25519_dalek::{PublicKey, SecretKey, Keypair};

use bech32::{FromBase32, ToBase32, Variant};

pub static SECRET_KEY_HRP: &'static str = "gs";
pub static PUBLIC_KEY_HRP: &'static str = "gp";
pub static URL_PREFIX: &'static str = "https://rib.rs?key=";

pub fn key_pair_from_secret_key(key: &str) -> Result<Keypair> {
    let secret_key = decode_secret_key(key)?;
    let public_key = PublicKey::from(&secret_key);
    let key_pair = Keypair {
        secret: secret_key,
        public: public_key,
    };
    Ok(key_pair)
}

pub fn keypair_to_url(keypair: &Keypair) -> Result<String> {
    let secret_key_string = encode_secret_key(&keypair.secret)?;
    let url = format!("{}{}", URL_PREFIX, secret_key_string);
    Ok(url)
}

pub fn url_to_keypair(url: &str) -> Result<Keypair> {
    if !url.starts_with(URL_PREFIX) {
        bail!("incorrect URL prefix for secret key");
    }

    let key = url.split_at(URL_PREFIX.len()).1;
    key_pair_from_secret_key(key)
}

pub fn encode_secret_key(key: &SecretKey) -> Result<String> {
    let bytes = key.as_bytes();
    let encoded = bech32::encode(SECRET_KEY_HRP, bytes.to_base32(), Variant::Bech32m).e()?;
    Ok(encoded)
}

pub fn decode_secret_key(key: &str) -> Result<SecretKey> {
    let (hrp, data, variant) = bech32::decode(key).e()?;

    if hrp != SECRET_KEY_HRP {
        bail!("wrong HRP in secret key decoding");
    }

    if variant != Variant::Bech32m {
        bail!("wrong bech32 variant in secret key decoding");
    }

    let bytes = Vec::<u8>::from_base32(&data).e()?;
    let key = SecretKey::from_bytes(&bytes).e()?;
    Ok(key)
}

pub fn encode_public_key(key: &PublicKey) -> Result<String> {
    let bytes = key.as_bytes();
    let encoded = bech32::encode(PUBLIC_KEY_HRP, bytes.to_base32(), Variant::Bech32m).e()?;
    Ok(encoded)
}

pub fn decode_public_key(key: &str) -> Result<PublicKey> {
    let (hrp, data, variant) = bech32::decode(key).e()?;

    if hrp != PUBLIC_KEY_HRP {
        bail!("wrong HRP in public key decoding");
    }

    if variant != Variant::Bech32m {
        bail!("wrong bech32 variant in public key decoding");
    }

    let bytes = Vec::<u8>::from_base32(&data).e()?;
    let key = PublicKey::from_bytes(&bytes).e()?;
    Ok(key)
}

trait ResultWrapper<T> {
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
}
