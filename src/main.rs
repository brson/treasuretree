#![allow(unused)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use anyhow::{anyhow, bail, Result};
use rocket::response::{content::Html, Responder, Content};
use rocket_contrib::{templates::Template, json::Json};
use rocket_contrib::serve::StaticFiles;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::fmt;
use treasure_qrcode::create_qr_code;
use treasure::Treasure;
use rocket::Data;
use std::fs::{self, File, DirEntry, Metadata};
use std::io::prelude::*;
use std::io::BufReader;
use rocket::http::{RawStr, Method, ContentType};
use std::time::SystemTime;

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
    let filename = format!("data/treasure/{key}", key = treasure_key);
    let return_url = format!("{host}/api/plant/{key}\n", host = "http://localhost:8000", key = treasure_key);

    fs::create_dir_all("data/treasure")?;

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

/// A treasure's image.
///
/// The `public_key` is bech32 encoded.
///
/// Need to set the mime/type.
/// For now set to image/jpeg.
#[get("/treasure-images/<public_key>")]
fn retrieve_treasure_image(public_key: &RawStr) -> Result<Content<Vec<u8>>> {
    let public_key = public_key.percent_decode()?;
    let public_key = crypto::decode_public_key(&public_key)?;
    let public_key = crypto::encode_public_key(&public_key)?;

    let path = format!("data/treasure/{}", public_key);
    let file = BufReader::new(File::open(path)?);
    let record: PlantInfoRequest = serde_json::from_reader(file)?;
    let encoded_image = record.image;
    let decoded_image = base64::decode(&encoded_image)?;

    // TODO: Correct content type
    Ok(Content(ContentType::JPEG, decoded_image))
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
struct ClaimInfoResponse {
    message: String,
    return_url: String,

}

/// Claim a treasure.
///
/// Checks that a treasure exists,
/// then verifies the signature with the public key and the nonce.
///
/// If the checks pass then store a record indicating
/// the treasure was claimed by the logged in user.
///
/// TODO: Add a user concept
#[post("/api/claim", format = "json", data = "<claim_info>")]
fn claim_treasure_with_key(claim_info: Json<ClaimInfoRequest>) -> Result<Json<ClaimInfoResponse>> {

    let res = (|| {
        // verify if it's a valid Public key
        let public_key_decode = crypto::decode_public_key(&claim_info.public_key)?;
        let public_key_encode = crypto::encode_public_key(&public_key_decode)?;

        dbg!(&claim_info);
        
        let filename = format!("data/treasure/{}", public_key_encode);
        if !Path::new(&filename).is_file() {
            bail!("Treasure doesn't exist")
        } else {
            let message = claim_info.nonce.as_bytes();
            let signature = crypto::decode_signature(&claim_info.signature)?;

            crypto::verify_signature(message, &signature, &public_key_decode)?;
            
            
            // todo:
            // - claim success and transfer asset 
            // - disable secret_key
            // - sync to blockchain

            let filename = format!("data/claim/{key}", key = public_key_encode);
            fs::create_dir_all("data/claim")?;

            println!("hihihi filename: {}", &filename);
            println!("hihihi claim info: {:?}", &claim_info.0);
            
            let mut file = File::create(filename)?;
            serde_json::to_writer(file, &claim_info.0)?;

            let return_url = format!("{host}/api/plant/{key}\n", host = "http://localhost:8000", key = public_key_encode);

            Ok(Json(ClaimInfoResponse {
                message: format!("Congrats! Treasure received!"),
                return_url,
            }))
        }
    })();
    dbg!(res)
    
}

#[get("/")]
fn root() -> Template {
    Template::render("index", json!({}))
}

#[get("/<page>")]
fn static_page(page: String) -> Template {
    Template::render(page, json!({}))
}

#[get("/recent", )]
fn recent_page() -> Result<Template> {

    fs::create_dir_all("treasure")?;

    // This nightmare expression collects DirEntrys for every
    // thing in the directory that is a file,
    // and extracting the modify time,
    // while also bubbling any possible errors.
    // It does the "collect Iter<Item = Result> into Result<Vec>" trick.
    let mut files = fs::read_dir("treasure")?
        // Get the file metadata
        .map(|dent: Result<DirEntry, _>| {
            dent.and_then(|dent| Ok((dent.metadata()?, dent)))
        })
        // Only keep entries that are files or errors
        .filter(|dent: &Result<(Metadata, DirEntry), _>| {
            dent.as_ref().map(|(meta, _)| meta.is_file()).unwrap_or(true)
        })
        // Keep modify time for sorting
        .map(|dent: Result<(Metadata, DirEntry), _> | {
            dent.and_then(|(meta, dent)| Ok((meta.modified()?, dent)))
        })
        // Collect iter of Result into Result<Vec>,
        // and return any error.
        .collect::<Result<Vec<_>, _>>()?;

    files.sort_by_key(|&(time, _)| time);

    #[derive(Serialize)]
    struct Treasure {
        public_key: String,
        image_url: String,
        date_time: String,
    }

    let treasures = files.into_iter().take(10).map(|(time, dent)| {
        let public_key = dent.file_name().into_string().expect("utf-8");
        let image_url = format!("treasure-images/{}", public_key);
        let date_time = chrono::DateTime::<chrono::Local>::from(time);
        let date_time = date_time.to_rfc2822();
        Treasure {
            public_key,
            image_url,
            date_time,
        }
    }).collect();

    #[derive(Serialize)]
    struct TemplateData {
        treasures: Vec<Treasure>,
    }

    let data = TemplateData {
        treasures,
    };

    Ok(Template::render("recent", data))
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
            recent_page,
            create_treasure_key,
            plant_treasure_with_key,
            retrieve_treasure,
            retrieve_treasure_image,
            claim_treasure_with_key,
        ])
        .launch();
}
