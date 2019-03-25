#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::env;

#[get("/")]
fn index() -> String {
    let token = env::var("MOVIEDB_TOKEN_v3").unwrap();
    format!("{}", token)
}

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    rocket::ignite().mount("/", routes![index]).launch();
}