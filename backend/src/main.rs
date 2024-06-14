#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate rocket;

use config::*;
use controller::*;
use rocket_okapi::openapi_get_routes;
use rocket_cors::{CorsOptions, AllowedOrigins};
use std::env;

mod config;
mod controller;
mod entities;
mod fairings;
mod model;
mod utils;

async fn init() {}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = init().await;
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error creating CORS");

    let _rocket = rocket::build()
        .attach(cors)
        .mount(
            "/api/v1",
            routes![
                controller::post_controller::upload,
                controller::post_controller::get_post,
                controller::post_controller::get_posts,
                controller::post_controller::new_post,
                controller::post_controller::delete_post_handler,
                controller::auth_controller::login,
            ],
        )
        .configure(rocket::Config::figment().merge(("port", server_port.parse::<u16>().unwrap())))
        .mount("/", rocket::fs::FileServer::from("public"))
        .launch()
        .await?;

    Ok(())
}
