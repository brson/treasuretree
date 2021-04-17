#![allow(unused)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use anyhow::{anyhow, bail, Result};
use rocket::response::content::Html;
use rocket_contrib::{templates::Template, json::Json};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::path::{Path, PathBuf};
use treasure_qrcode::create_qr_code;
use treasure::Treasure;

use rocket::Data;
use std::fs::File;
use rocket::http::RawStr;

mod crypto;
mod treasure_qrcode;
mod treasure;

#[derive(Debug, Serialize)]
pub struct UniqueCodeJson {
    hex: String,
    qrcode: String,
    url: String,
}

#[get("/api/create")]
fn create_treasure_key() -> Json<UniqueCodeJson> {
    let init_keys = create_qr_code();
    let first_key = &init_keys[0];

    let first_key = UniqueCodeJson {
        hex: first_key.hex.clone(),
        // 50 is the size, bigger number means smaller size on the page
        qrcode: first_key.qrcode.to_svg_string(50), 
        url: first_key.url.clone(),
    };

    Json(first_key)
}

#[derive(Deserialize)]
struct PlantInfoRequest {
    /// An image, base64 encoded
    base64_image: String,
    /// A private key, hex encoded
    ///
    /// FIXME: Should be a pub-key, derived on the client
    private_key: String,
}

/// Stores a treasure and associated key
///
/// Stores the json to disk,
/// with the private key (pubkey in the future) as the name of the file.
/// The key can be used later to retrieve (or claim) the treasure.
#[post("/api/plant", data = "<plant_info>")]
fn plant_treasure_with_key(plant_info: Json<PlantInfoRequest>) -> Result<()> {
    panic!()
}

/// Return an html page displaying a treasure
///
/// `private_key` is hex encoded.
///
/// The page includes an `img` tag with the url of the treasure image,
/// and displays the private (public) key of the treasure.
///
/// Remember to percent-decode the rawstr.
///
/// Load the template from templates/treasure/template.html.tera.
#[get("/treasure/<private_key>")]
fn retrieve_treasure(private_key: &RawStr) -> Result<Template> {
    panic!()
}

/// A treasure's pic.
///
/// The `private_key` is hex encoded.
///
/// Need to set the mime/type.
/// For now set to image/jpeg.
#[get("/treasure-pics/<private_key>")]
fn retrieve_treasure_pic(private_key: &RawStr) -> Result<File> {
    panic!()
}

#[post("/api/upload-test", data = "<paste>")]
fn upload_test(paste: Data) -> Result<String, std::io::Error> {
    let new_treasure = Treasure::new(12);
    let filename = format!("treasure/{treasure_id}", treasure_id = new_treasure);
    let url = format!("{host}/treasure/{treasure_id}\n", host = "http://localhost:8000", treasure_id = new_treasure);
    println!("{}", &url);
    paste.stream_to_file(Path::new(&filename))?;

    Ok(url)
}

#[get("/treasure-test/<treasure_id>")]
fn retrieve_treasure_test(treasure_id: &RawStr) -> Option<File> {
    let filename = format!("treasure/{treasure_id}", treasure_id = treasure_id);
    File::open(&filename).ok()
}

#[post("/api/claim")]
fn claim_treasure_with_key() -> String {
    "claim".to_string()
}

#[get("/")]
fn root() -> Template {
    Template::render("index", json!({}))
}

#[get("/<page>")]
fn static_page(page: String) -> Template {
    Template::render(page, json!({}))
}

#[get("/css/<file>")]
fn static_css(file: String) -> Template {
    let file = &Path::new(&file);

    assert!(file.file_name().is_some());

    let template_name = file.with_extension("");
    let template_name = format!("css/{}", template_name.display());
    Template::render(template_name, json!({}))
}

#[get("/js/<file>")]
fn static_js(file: String) -> Template {
    let file = &Path::new(&file);

    assert!(file.file_name().is_some());

    let template_name = file.with_extension("");
    let template_name = format!("js/{}", template_name.display());
    Template::render(template_name, json!({}))
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![
            root,
            static_page,
            static_css,
            static_js,
            create_treasure_key,
            plant_treasure_with_key,
            retrieve_treasure,
            retrieve_treasure_pic,
            upload_test,
            retrieve_treasure_test,
            claim_treasure_with_key,
        ])
        .launch();
}
