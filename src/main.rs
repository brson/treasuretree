#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate include_dir;

use std::path::{PathBuf, Path};
use anyhow::{Result, anyhow, bail};
use include_dir::Dir;
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
fn root() -> Html<String> {
    let file = STATIC_FILES.get_file("index.html").unwrap();
    Html(file.contents_utf8().unwrap().to_string())
}

#[get("/<page>")]
fn static_page(page: String) -> Result<Html<String>> {
    let path = &Path::new(&page);
    let path = path.with_extension("html");

    let file = STATIC_FILES.get_file(&path)
        .ok_or_else(|| anyhow!("not found"))?;

    let content = file.contents_utf8().expect("utf8");

    Ok(Html(content.to_owned()))
}

#[get("/<path..>", rank = 0)]
fn static_file(path: PathBuf) -> Result<StaticResponder> {

    assert!(path.is_relative());

    let ext = path.extension()
        .map(|ostr| ostr.to_str())
        .flatten();

    // HTML content is served from URI's with no file extension
    if ext == Some("html") {
        bail!("not found");
    }

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
            root,
            static_page,
            static_file,
            create_treasure_key,
            plant_treasure_with_key,
            claim_treasure_with_key,
        ])
        .launch();
}
