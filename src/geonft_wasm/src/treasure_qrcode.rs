use geonft_nostd::crypto;
use anyhow::Result;
use crypto::ResultWrapper;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct UniqueCode {
    pub secret_key: String,
    pub qrcode: String,
    pub url: String,
}

pub fn init_random_qrcode() -> Result<UniqueCode> {
    let keypair = super::new_keypair();
    let secret_key_string = crypto::encode_treasure_secret_key(&keypair.secret)?;
    let url = crypto::keypair_to_treasure_secret_url(&keypair)?;
    let qrcode = QrCode::encode_text(&url, QrCodeEcc::Low).unwrap();

    Ok(UniqueCode {
        secret_key: secret_key_string,
        // Argument is the size, bigger number means smaller size on the page
        qrcode: qrcode.to_svg_string(0),
        url,
    })
}
