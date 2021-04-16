#![allow(unused)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use anyhow::{anyhow, bail, Result};
use rocket::response::content::Html;
use rocket_contrib::{templates::Template, json::Json};
use serde::Serialize;
use serde_json::json;
use std::path::{Path, PathBuf};
use treasure_qrcode::create_qr_code;
use treasure::Treasure;

use rocket::Data;
use std::fs::File;
use rocket::http::RawStr;

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

#[post("/api/plant", data = "<paste>")]
fn plant_treasure_with_key(paste: Data) -> Result<String, std::io::Error> {
    let new_treasure = Treasure::new(12);
    let filename = format!("treasure/{treasure_id}", treasure_id = new_treasure);
    let url = format!("{host}/treasure/{treasure_id}\n", host = "http://localhost:8000", treasure_id = new_treasure);
    println!("{}", &url);
    paste.stream_to_file(Path::new(&filename))?;

    Ok(url)
}

#[get("/treasure/<treasure_id>")]
fn retrieve_treasure(treasure_id: &RawStr) -> Option<File> {
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
            claim_treasure_with_key,
            retrieve_treasure
        ])
        .launch();
}
