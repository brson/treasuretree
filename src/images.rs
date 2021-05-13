use image::ImageFormat;
use rocket::http::ContentType;
use std::str;

pub fn detect_image_type(data: &[u8]) -> Option<ContentType> {
    let maybe_image_format = image::guess_format(data);
    match maybe_image_format {
        Ok(ImageFormat::Png) => Some(ContentType::PNG),
        Ok(ImageFormat::Jpeg) => Some(ContentType::JPEG),
        _ => match str::from_utf8(data) {
            Ok(text) => {
                if text.contains("svg") {
                    Some(ContentType::SVG)
                } else {
                    None
                }
            }
            _ => None,
        },
    }
}
