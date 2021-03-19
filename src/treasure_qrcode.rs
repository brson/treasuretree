use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;

pub fn create_qr_code() -> String {
    let qr = QrCode::encode_text("Hello, world!",
                                 QrCodeEcc::Medium).unwrap();
    let svg = qr.to_svg_string(4);

    println!("hello,,,,,,");
    String::from("OK")
}

