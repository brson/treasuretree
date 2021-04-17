mod utils;

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

pub struct KeyInfo {
    secret: String,
    public: String,
}

#[wasm_bindgen]
pub fn secret_url_to_key_info(url: &str) -> KeyInfo {
    panic!()
}
