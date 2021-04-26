#![allow(unused)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use anyhow::{anyhow, bail, Result};
use rocket::response::{content::Html, Responder};
use rocket_contrib::{templates::Template, json::Json};
use rocket_contrib::serve::StaticFiles;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::fmt;
use treasure_qrcode::create_qr_code;
use treasure::Treasure;
use rocket::Data;
use std::fs::{self, File};
use std::io::prelude::*;
use rocket::http::{RawStr, Method};

mod crypto;
mod treasure_qrcode;
mod treasure;

#[derive(Debug, Serialize)]
pub struct UniqueCodeJson {
    secret_key: String,
    qrcode: String,
    url: String,
}

#[get("/api/create")]
fn create_treasure_key() -> Result<Json<UniqueCodeJson>> {
    let init_keys = create_qr_code()?;
    let first_key = &init_keys[0];

    let first_key = UniqueCodeJson {
        secret_key: first_key.secret_key.clone(),
        // Argument is the size, bigger number means smaller size on the page
        qrcode: first_key.qrcode.to_svg_string(0), 
        url: first_key.url.clone(),
    };

    Ok(Json(first_key))
}

#[derive(Serialize, Deserialize, Debug)]
struct PlantInfoRequest {
    /// An image, base64 encoded
    image: String,
    /// A public key to represent the treasure, bech32 encoded
    public_key: String,
    /// A signature against the base64 encoded image by the corresponding private key
    signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlantInfoResponse {
    return_url: String,
}

/// Stores a treasure and associated key
///
/// Stores the json to disk,
/// with the encoded pubkey as the name of the file.
/// The key can be used later to retrieve (or claim) the treasure.
///
/// # Errors
///
/// If the signature is not a valid signature of the provided image with
/// the provided public key.
#[post("/api/plant", format = "json", data = "<plant_info>")]
fn plant_treasure_with_key(plant_info: Json<PlantInfoRequest>) -> Result<Json<PlantInfoResponse>> {
    let treasure_key = &plant_info.public_key;
    let filename = format!("treasure/{key}", key = treasure_key);
    let return_url = format!("{host}/api/plant/{key}\n", host = "http://localhost:8000", key = treasure_key);

    fs::create_dir_all("treasure")?;

    let mut file = File::create(filename)?;
    serde_json::to_writer(file, &plant_info.0)?;
    
    let res = PlantInfoResponse {
        return_url,
    };
    
    Ok(Json(res))
}

/// Return an html page displaying a treasure
///
/// `public_key` is bech32 encoded.
///
/// The page includes an `img` tag with the url of the treasure image,
/// and displays the private (public) key of the treasure.
///
/// Remember to percent-decode the rawstr.
///
/// Load the template from templates/treasure/template.html.tera.
#[get("/treasure/<public_key>")]
fn retrieve_treasure(public_key: &RawStr) -> Result<Template> {
    panic!()
}

/// A treasure's pic.
///
/// The `public_key` is bech32 encoded.
///
/// Need to set the mime/type.
/// For now set to image/jpeg.
#[get("/treasure-pics/<public_key>")]
fn retrieve_treasure_pic(public_key: &RawStr) -> Result<File> {
    panic!()
}

#[derive(Serialize, Deserialize, Debug)]
struct ClaimInfoRequest {
    /// A random string signed by the private key as evidence of ownership
    nonce: String,
    /// The public key of the treasure, bech32 encoded
    public_key: String,
    /// A signature against the nonce by the corresponding private key
    signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClaimInfoResponse;

#[post("/api/claim", format = "json", data = "<claim_info>")]
fn claim_treasure_with_key(claim_info: Json<ClaimInfoRequest>) -> Result<Json<ClaimInfoResponse>> {
    panic!()
}

#[get("/")]
fn root() -> Template {
    Template::render("index", json!({}))
}

#[get("/<page>")]
fn static_page(page: String) -> Template {
    Template::render(page, json!({}))
}

fn main() {
    let css_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
    let js_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
    let images_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/images");
    let wasm_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/wasm/pkg");
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/css", StaticFiles::from(css_dir))
        .mount("/js", StaticFiles::from(js_dir))
        .mount("/images", StaticFiles::from(images_dir))
        .mount("/wasm/pkg", StaticFiles::from(wasm_dir))
        .mount("/", routes![
            root,
            static_page,
            create_treasure_key,
            plant_treasure_with_key,
            retrieve_treasure,
            retrieve_treasure_pic,
            claim_treasure_with_key,
        ])
        .launch();
}
