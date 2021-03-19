#![allow(unused)]

#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;

use std::path::{PathBuf, Path};
use anyhow::{Result, anyhow, bail};
use rocket::response::content::Html;
use rocket_contrib::templates::Template;
use serde::Serialize;
use serde_json::json;

#[get("/api/create")]
fn create_treasure_key() -> String {
    "create".to_string()
}

#[post("/api/plant")]
fn plant_treasure_with_key() -> String {
    "plant".to_string()
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

mod treasure_qrcode;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![
            root,
            static_page,
            static_css,
            create_treasure_key,
            plant_treasure_with_key,
            claim_treasure_with_key,
        ])
        .launch();
}



