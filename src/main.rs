#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate include_dir;

use std::path::{PathBuf, Path};
use anyhow::{Result, anyhow};
use include_dir::Dir;
use log::info;
use rocket::response::content::Html;

static STATIC_FILES: Dir = include_dir!("./static");

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
fn static_root() -> Html<String> {
    let file = STATIC_FILES.get_file("index.html").unwrap();
    Html(file.contents_utf8().unwrap().to_string())
}

#[get("/<path..>")]
fn static_file(path: PathBuf) -> Result<StaticResponder> {

    let path = if path == Path::new("") {
        PathBuf::from("/index.html")
    } else {
        path
    };

    let ext = path.extension()
        .map(|ostr| ostr.to_str())
        .flatten();

    let file = STATIC_FILES.get_file(&path)
        .ok_or_else(|| anyhow!("not found"))?;

    let content = file.contents().to_vec();
    let content_type = ext.map(|ext| ContentType::from_extension(ext))
        .flatten()
        .ok_or_else(|| anyhow!("can't determine content type"))?;

    Ok(StaticResponder {
        content,
        content_type,
    })
}

use rocket::http::ContentType;
use rocket::response::Responder;

#[derive(Responder)]
struct StaticResponder {
    content: Vec<u8>,
    content_type: ContentType,
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            create_treasure_key,
            plant_treasure_with_key,
            claim_treasure_with_key,
            static_root,
            static_file,
        ])
        .launch();
}
