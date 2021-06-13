#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;

use geonft_request::PlantRequest;
use geonft_shared::io::{self, TreasureTemplateData};
use std::fs::{self, DirEntry, File, Metadata};
use std::io::BufReader;
use std::collections::HashMap;
use rocket_dyn_templates::Template;
use rocket::serde::{Serialize, json::json};
use rocket::fs::FileServer;

// mod api;
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


#[launch]
fn rocket() -> _ {
    let css_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
    let js_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
    let images_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/images");
    let wasm_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../geonft_wasm/pkg");

    rocket::build()
        .attach(Template::fairing())
        .mount("/css", FileServer::from(css_dir))
        .mount("/js", FileServer::from(js_dir))
        .mount("/images", FileServer::from(images_dir))
        .mount("/wasm/pkg", FileServer::from(wasm_dir))
        .mount("/",
               routes![
                   root_page,
                   static_page,
])        
}
