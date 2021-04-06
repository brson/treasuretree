use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;
use rand::prelude::*;
use std::fmt;

pub struct UniqueCode {
    hex: Vec<u8>,
    qrcode: QrCode,
    url: String,
}

impl fmt::Debug for UniqueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UniqueCode")
            .field("hex", &self.hex)
            .field("qrcode", &"qrcode_placeholder")
            .field("url", &self.url)
            .finish()
    }
}

pub fn create_qr_code() -> Vec<UniqueCode> {
    init_random_qrcode(10)
}

fn init_random_qrcode(quantity: i32) -> Vec<UniqueCode> {
    let mut rng = rand::thread_rng();
    let mut qrcodes = Vec::new();

    for i in 0..quantity {
        let mut nums = [0u8; 20];
        rng.fill(&mut nums[..]);

        let qr = QrCode::encode_binary(&nums, QrCodeEcc::Low).unwrap();

        qrcodes.push(UniqueCode {
            hex: nums.to_vec(),
            qrcode: qr, // qr.to_svg_string(2),
            url: "placeholder".to_string(), // "domain?key=".to_string() + &nums,
        });
    }

    println!("{:#?}", &qrcodes);
    qrcodes
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
