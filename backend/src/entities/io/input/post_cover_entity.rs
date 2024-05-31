use std::io::{self, Read};

use serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::request::{self, Request};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::outcome::Outcome;

#[derive(Debug)]
pub enum Error {
    InvalidTitle,
    InvalidBody,
}

#[derive(Serialize, Deserialize)]
pub struct PostCoverInput {
    pub post_id: i32,
    pub headline: String,
    pub description: String,
    pub image_path: String,
}
