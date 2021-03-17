#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;

#[get("/api/create")]
fn create_treasure_key() -> String {
    panic!()
}

#[post("/api/plant")]
fn plant_treasure_with_key() -> String {
    panic!()
}

#[post("/api/claim")]
fn claim_treasure_with_key() -> String {
    panic!()
}

#[get("/")]
fn static_root() -> String {
    panic!()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            create_treasure_key,
            plant_treasure_with_key,
            claim_treasure_with_key,
        ])
        .launch();
}
