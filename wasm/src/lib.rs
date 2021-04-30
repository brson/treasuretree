mod utils;

use wasm_bindgen::prelude::*;
use anyhow::Result;
use ed25519_dalek::{PublicKey, SecretKey, Keypair};
use bech32::{FromBase32, ToBase32, Variant};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}

#[path = "../../src/crypto_shared.rs"]
mod crypto_shared;
use crypto_shared as crypto;

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
    
    Some("fixme bogus signature".to_string())
}
