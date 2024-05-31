#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use] 
extern crate rocket;

use std::env;
use controller::*;
use config::*;
use rocket_okapi::openapi_get_routes;

mod config;
mod model;
mod controller;
mod entities;
mod utils;
mod fairings;

async fn init() {
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = init().await;

    let _rocket = rocket::build()
    .attach(fairings::cors::CORS)
    .mount("/api/v1", routes![
        controller::post_cover_controller::get_cover_posts,
        controller::post_controller::get_post,
        controller::post_controller::get_posts,
        controller::post_controller::new_post,
        controller::auth_controller::login,
    ])
    .mount("/", rocket::fs::FileServer::from("public"))
    .launch()
    .await?;

    Ok(())
}