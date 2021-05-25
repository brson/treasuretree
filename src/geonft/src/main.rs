#![allow(unused)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use anyhow::{anyhow, bail, Result};
use geonft_shared::data;
use rocket::http::{ContentType, Method, RawStr};
use rocket::response::{content::Html, Content, Responder};
use rocket::Data;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::{json::Json, templates::Template};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use std::fs::{self, DirEntry, File, Metadata};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use treasure::Treasure;

mod api;
mod crypto;
mod images;
mod treasure;

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
    fs::create_dir_all(data::PLANT_DIR)?;

    // This nightmare expression collects DirEntrys for every
    // thing in the directory that is a file,
    // and extracting the modify time,
    // while also bubbling any possible errors.
    // It does the "collect Iter<Item = Result> into Result<Vec>" trick.
    let mut files = fs::read_dir(data::PLANT_DIR)?
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

    files.sort_by_key(|&(time, _)| time);

    let treasures = files
        .into_iter()
        .take(10)
        .map(|(time, dent)| {
            let public_key = dent.file_name().into_string().expect("utf-8");
            let public_url = format!("treasure/{}", public_key);
            let image_url = format!("treasure-images/{}", public_key);
            let planted_date_time = chrono::DateTime::<chrono::Local>::from(time);
            let planted_date_time = planted_date_time.to_rfc2822();
            let planted_by = "todo".to_string();
            let claimed_date_time = "todo".to_string();
            let claimed_by = "todo".to_string();
            TreasureTemplateData {
                public_key,
                public_url,
                image_url,
                planted_date_time,
                planted_by,
                claimed_date_time,
                claimed_by,
            }
        })
        .collect();

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

    let path = format!("{}/{}", data::PLANT_DIR, public_key);
    let file = fs::metadata(path)?;
    let time = file.modified()?;
    let planted_date_time = chrono::DateTime::<chrono::Local>::from(time);
    let planted_date_time = planted_date_time.to_rfc2822();

    let public_url = format!("treasure/{}", public_key);
    let image_url = format!("treasure-images/{}", public_key);

    let planted_by = "todo".to_string();
    let claimed_date_time = "todo".to_string();
    let claimed_by = "todo".to_string();

    #[derive(Serialize)]
    struct TemplateData {
        base_href: &'static str,
        treasure: TreasureTemplateData,
    }

    let data = TemplateData {
        base_href: "..",
        treasure: TreasureTemplateData {
            public_key,
            public_url,
            image_url,
            planted_date_time,
            planted_by,
            claimed_date_time,
            claimed_by,
        },
    };

    Ok(Template::render("treasure", data))
}

#[derive(Serialize)]
struct TreasureTemplateData {
    public_key: String,
    public_url: String,
    image_url: String,
    planted_date_time: String,
    planted_by: String,
    claimed_date_time: String,
    claimed_by: String,
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

    let path = format!("{}/{}", data::PLANT_DIR, public_key);
    let file = BufReader::new(File::open(path)?);
    let record: api::PlantRequest = serde_json::from_reader(file)?;
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
            ],
        )
        .launch();
}
