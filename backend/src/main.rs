#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use] 
extern crate rocket;

use std::env;
use controller::*;
use config::*;

mod config;
mod model;
mod controller;
mod entities;



async fn init() {
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = init().await;

    let _rocket = rocket::build()
    .mount("/api/v1", routes![
        controller::post_controller::get_posts,
        controller::post_controller::new_post,
    ])
    .mount("/", rocket::fs::FileServer::from("public"))
    .launch()
    .await?;

    Ok(())
}