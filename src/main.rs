#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;

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
fn static_root() -> String {
    "hi".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            create_treasure_key,
            plant_treasure_with_key,
            claim_treasure_with_key,
            static_root,
        ])
        .launch();
}
