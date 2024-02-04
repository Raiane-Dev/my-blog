#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use] extern crate rocket;

use std::env;
use controller::*;
use config::*;

mod config;
mod model;
mod controller;
mod entities;


#[get("/world")]
fn wold() -> &'static str {
    "hello world"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
    .mount("/api/v1", routes![wold])
    .launch()
    .await?;

    Ok(())
}