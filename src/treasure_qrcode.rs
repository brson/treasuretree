use anyhow::Result;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;
use std::fmt;

use crate::crypto;

pub struct UniqueCode {
    pub secret_key: String,
    pub qrcode: QrCode,
    pub url: String,
}

impl fmt::Debug for UniqueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UniqueCode")
            .field("secret_key", &self.secret_key)
            .field("qrcode", &"qrcode_placeholder")
            .field("url", &self.url)
            .finish()
    }
}

pub fn create_qr_code() -> Result<Vec<UniqueCode>> {
    init_random_qrcode(10)
}

fn init_random_qrcode(quantity: i32) -> Result<Vec<UniqueCode>> {
    let mut qrcodes = Vec::new();

    for i in 0..quantity {
        let keypair = crypto::new_keypair();
        let secret_key_string = crypto::encode_secret_key(&keypair.secret)?;
        let url = crypto::keypair_to_url(&keypair)?;
        let qrcode = QrCode::encode_text(&url, QrCodeEcc::Low).unwrap();
        
        qrcodes.push(UniqueCode {
            secret_key: secret_key_string,
            qrcode,
            url,
        });
    }

    println!("{:#?}", &qrcodes);
    Ok(qrcodes)
}

// Prints the given QrCode object to the console.
fn print_qr(qr: &QrCode) {
    let border: i32 = 4;
    for y in -border..qr.size() + border {
        for x in -border..qr.size() + border {
            let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
            print!("{0}{0}", c);
        }
        println!();
    }
    println!();
}
