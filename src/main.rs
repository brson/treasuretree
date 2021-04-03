#![allow(unused)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use anyhow::{anyhow, bail, Result};
use rocket::response::content::Html;
use rocket_contrib::templates::Template;
use serde::Serialize;
use serde_json::json;
use std::path::{Path, PathBuf};

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

mod treasure_qrcode;

fn main() {
    treasure_qrcode::create_qr_code();
    /*
        rocket::ignite()
            .attach(Template::fairing())
            .mount("/", routes![
                root,
                static_page,
                create_treasure_key,
                plant_treasure_with_key,
                claim_treasure_with_key,
            ])
            .launch();
    */
}
