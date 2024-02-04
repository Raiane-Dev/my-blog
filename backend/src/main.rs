#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use] 
extern crate rocket;

use std::env;
use controller::*;
use config::*;

mod config;
mod model;
mod controller;
mod entities;



fn init() {

}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    self::init();

    let _rocket = rocket::build()
    .mount("/api/v1", routes![controller::post_controller::get_posts])
    .mount("/*", rocket::fs::FileServer::from("public"))
    .launch()
    .await?;

    Ok(())
}