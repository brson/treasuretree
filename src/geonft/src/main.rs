//! The geonft Rocket web application

#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use anyhow::Result;
use geonft_request::PlantRequest;
use geonft_shared::io::{self, TreasureTemplateData};
use rocket::http::{ContentType, RawStr};
use rocket::response::Content;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;
use serde_json::json;
use std::fs::{self, DirEntry, File, Metadata};
use std::io::BufReader;

mod api;
mod crypto;
mod images;

#[get("/")]
fn root_page() -> Template {
    Template::render("index", json!({}))
}

#[get("/<page>")]
fn static_page(page: String) -> Template {
    Template::render(page, json!({}))
}

#[get("/recent")]
fn recent_page() -> Result<Template> {
    fs::create_dir_all(io::PLANT_DIR)?;

    // This nightmare expression collects DirEntrys for every
    // thing in the directory that is a file,
    // and extracting the modify time,
    // while also bubbling any possible errors.
    // It does the "collect Iter<Item = Result> into Result<Vec>" trick.
    let mut files = fs::read_dir(io::PLANT_DIR)?
        // Get the file metadata
        .map(|dent: Result<DirEntry, _>| dent.and_then(|dent| Ok((dent.metadata()?, dent))))
        // Only keep entries that are files or errors
        .filter(|dent: &Result<(Metadata, DirEntry), _>| {
            dent.as_ref()
                .map(|(meta, _)| meta.is_file())
                .unwrap_or(true)
        })
        // Keep modify time for sorting
        .map(|dent: Result<(Metadata, DirEntry), _>| {
            dent.and_then(|(meta, dent)| Ok((meta.modified()?, dent)))
        })
        // Collect iter of Result into Result<Vec>,
        // and return any error.
        .collect::<Result<Vec<_>, _>>()?;

    // Sort by time, reversed
    files.sort_by(|&(time1, _), &(time2, _)| time2.cmp(&time1));

    let mut treasures = Vec::new();

    for (time, dent) in files.into_iter().take(10) {
        let public_key = dent.file_name().into_string().expect("utf-8");
        let treasure = io::load_treasure_data(&public_key)?;
        treasures.push(treasure);
    }

    #[derive(Serialize)]
    struct TemplateData {
        treasures: Vec<TreasureTemplateData>,
    }

    let data = TemplateData { treasures };

    Ok(Template::render("recent", data))
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
fn treasure_page(public_key: &RawStr) -> Result<Template> {
    let public_key = public_key.percent_decode()?;
    let public_key = crypto::decode_treasure_public_key(&public_key)?;
    let public_key = crypto::encode_treasure_public_key(&public_key)?;

    let treasure = io::load_treasure_data(&public_key)?;

    #[derive(Serialize)]
    struct TemplateData {
        base_href: &'static str,
        treasure: TreasureTemplateData,
    }

    let data = TemplateData {
        base_href: "..",
        treasure,
    };

    Ok(Template::render("treasure", data))
}

/// A treasure's image.
///
/// The `public_key` is bech32 encoded.
///
/// Need to set the mime/type.
/// For now set to image/jpeg.
#[get("/treasure-images/<public_key>")]
fn treasure_image(public_key: &RawStr) -> Result<Content<Vec<u8>>> {
    let public_key = public_key.percent_decode()?;
    let public_key = crypto::decode_treasure_public_key(&public_key)?;
    let public_key = crypto::encode_treasure_public_key(&public_key)?;

    let path = format!("{}/{}", io::PLANT_DIR, public_key);
    let file = BufReader::new(File::open(path)?);
    let record: PlantRequest = serde_json::from_reader(file)?;
    let encoded_image = record.image;
    let decoded_image = base64::decode(&encoded_image)?;

    let content_type = images::detect_image_type(&decoded_image).unwrap_or(ContentType::Binary);

    Ok(Content(content_type, decoded_image))
}

fn main() {
    let css_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
    let js_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
    let images_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/images");
    let wasm_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../geonft_wasm/pkg");
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/css", StaticFiles::from(css_dir))
        .mount("/js", StaticFiles::from(js_dir))
        .mount("/images", StaticFiles::from(images_dir))
        .mount("/wasm/pkg", StaticFiles::from(wasm_dir))
        .mount(
            "/",
            routes![
                root_page,
                static_page,
                recent_page,
                treasure_page,
                treasure_image,
                api::plant_treasure_with_key,
                api::claim_treasure_with_key,
                api::treasure_exists,
            ],
        )
        .launch();
}
