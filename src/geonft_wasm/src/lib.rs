extern crate alloc;

use alloc::format;

mod treasure_qrcode;
mod utils;
use geonft_nostd::crypto::{self, Keypair};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

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
pub fn create_qrcode() -> Option<String> {
    let new_qrcode = treasure_qrcode::init_random_qrcode().ok();
    serde_json::to_string(&new_qrcode).ok()
}

#[wasm_bindgen]
pub fn sanity_check_treasure_secret_url(url: &str) -> bool {
    url.starts_with(crypto::TREASURE_SECRET_PLANT_URL_PREFIX)
        || url.starts_with(crypto::TREASURE_SECRET_CLAIM_URL_PREFIX)
        || url.starts_with(crypto::TREASURE_SECRET_PLANT_URL_PREFIX_LOCAL)
        || url.starts_with(crypto::TREASURE_SECRET_CLAIM_URL_PREFIX_LOCAL)
}

#[wasm_bindgen]
pub fn treasure_secret_url_to_secret_key(url: &str) -> Option<String> {
    crypto::treasure_secret_url_to_keypair(url)
        .ok()
        .map(|kp| kp.secret)
        .map(|key| crypto::encode_treasure_secret_key(&key).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn treasure_secret_url_to_public_key(url: &str) -> Option<String> {
    crypto::treasure_secret_url_to_keypair(url)
        .ok()
        .map(|kp| kp.public)
        .map(|key| crypto::encode_treasure_public_key(&key).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn treasure_secret_key_to_public_key(key: &str) -> Option<String> {
    crypto::keypair_from_treasure_secret_key(key)
        .ok()
        .map(|kp| kp.public)
        .map(|key| crypto::encode_treasure_public_key(&key).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn treasure_secret_key_to_secret_claim_url(key: &str) -> Option<String> {
    crypto::keypair_from_treasure_secret_key(key)
        .ok()
        .map(|kp| crypto::keypair_to_treasure_secret_claim_url(&kp).ok())
        .flatten()
}

#[wasm_bindgen]
pub fn sign_plant_with_treasure_secret_key(
    treasure_secret_key: &str,
    account_public_key: &str,
    treasure_hash: &str,
) -> Option<String> {
    let treasure_secret_key = crypto::decode_treasure_secret_key(treasure_secret_key).ok()?;
    let account_public_key = crypto::decode_account_public_key(account_public_key).ok()?;

    let signature = crypto::sign_plant_request_for_treasure(
        &treasure_secret_key,
        &account_public_key,
        treasure_hash.as_bytes(),
    )
    .ok()?;

    crypto::encode_signature(&signature).ok()
}

#[wasm_bindgen]
pub fn sign_plant_with_account_secret_key(
    account_secret_key: &str,
    treasure_public_key: &str,
) -> Option<String> {
    let account_secret_key = crypto::decode_account_secret_key(account_secret_key).ok()?;
    let treasure_public_key = crypto::decode_treasure_public_key(treasure_public_key).ok()?;

    let signature =
        crypto::sign_plant_request_for_account(&account_secret_key, &treasure_public_key).ok()?;

    crypto::encode_signature(&signature).ok()
}

#[wasm_bindgen]
pub fn sign_claim_with_treasure_secret_key(
    treasure_secret_key: &str,
    account_public_key: &str,
) -> Option<String> {
    let treasure_secret_key = crypto::decode_treasure_secret_key(treasure_secret_key).ok()?;
    let account_public_key = crypto::decode_account_public_key(account_public_key).ok()?;

    let signature =
        crypto::sign_claim_request_for_treasure(&treasure_secret_key, &account_public_key).ok()?;

    crypto::encode_signature(&signature).ok()
}

#[wasm_bindgen]
pub fn sign_claim_with_account_secret_key(
    account_secret_key: &str,
    treasure_public_key: &str,
) -> Option<String> {
    let account_secret_key = crypto::decode_account_secret_key(account_secret_key).ok()?;
    let treasure_public_key = crypto::decode_treasure_public_key(treasure_public_key).ok()?;

    let signature =
        crypto::sign_claim_request_for_account(&account_secret_key, &treasure_public_key).ok()?;

    crypto::encode_signature(&signature).ok()
}

#[wasm_bindgen]
pub fn new_account_secret_key() -> Option<String> {
    let keypair = new_keypair();
    crypto::encode_account_secret_key(&keypair.secret).ok()
}

#[wasm_bindgen]
pub fn account_secret_key_to_public_key(key: &str) -> Option<String> {
    crypto::keypair_from_account_secret_key(key)
        .ok()
        .map(|kp| kp.public)
        .map(|key| crypto::encode_account_public_key(&key).ok())
        .flatten()
}

fn new_keypair() -> Keypair {
    crypto::generate_keypair(&mut OsRng)
}

#[wasm_bindgen]
pub fn get_hash(data: &str) -> Option<String> {
    crypto::get_hash(data).ok()
}

#[wasm_bindgen]
pub fn treasure_public_key_to_treasure_url(key: &str) -> String {
    format!("treasure/{}", key)
}

#[wasm_bindgen]
pub fn treasure_public_key_to_abbrev(key: &str) -> String {
    geonft_nostd::abbrev_pubkey(key)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
