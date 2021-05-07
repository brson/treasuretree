mod utils;

use wasm_bindgen::prelude::*;
use anyhow::Result;
use ed25519_dalek::{PublicKey, SecretKey, Keypair, Signer};
use bech32::{FromBase32, ToBase32, Variant};
use rand::rngs::OsRng;
use sha256::digest_bytes;

#[path = "../../src/crypto_shared.rs"]
mod crypto_shared;
use crypto_shared as crypto;
use crypto::ResultWrapper;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn sanity_check_url(url: &str) -> bool {
    url.starts_with(crypto::URL_PREFIX)
}

#[wasm_bindgen]
pub fn secret_url_to_secret_key(url: &str) -> Option<String> {
    crypto::secret_url_to_keypair(url).ok()
        .map(|kp| kp.secret)
        .map(|key| crypto::encode_secret_key(&key).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn secret_url_to_public_key(url: &str) -> Option<String> {
    crypto::secret_url_to_keypair(url).ok()
        .map(|kp| kp.public)
        .map(|key| crypto::encode_public_key(&key).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn secret_key_to_public_key(key: &str) -> Option<String> {
    crypto::keypair_from_secret_key(key).ok()
        .map(|kp| kp.public)
        .map(|key| crypto::encode_public_key(&key).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn secret_key_to_secret_url(key: &str) -> Option<String> {
    crypto::keypair_from_secret_key(key).ok()
        .map(|kp| crypto::keypair_to_secret_url(&kp).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn sign_with_secret_key(key: &str, data: &str) -> Option<String> {
    crypto::keypair_from_secret_key(&key).ok()
        .map(|kp| kp.try_sign(data.as_bytes()).ok())
        .flatten()
        .map(|key| crypto::encode_signature(&key).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn new_account_secret_key() -> Option<String> {
    let keypair = new_keypair();
    crypto::encode_account_secret_key(&keypair.secret).ok()
}

#[wasm_bindgen]
pub fn account_secret_key_to_public_key(key: &str) -> Option<String> {
    crypto::keypair_from_account_secret_key(key).ok()
        .map(|kp| kp.public)
        .map(|key| crypto::encode_account_public_key(&key).ok())
        .flatten()
}

fn new_keypair() -> Keypair {
    Keypair::generate(&mut OsRng)
}

#[wasm_bindgen]
pub fn get_hash(data: &[u8]) -> Option<String> {
    Some(digest_bytes(data))
}
