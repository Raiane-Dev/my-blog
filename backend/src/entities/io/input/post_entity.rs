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
pub struct PostInput {
    pub title: String, 
    pub body: String,
    pub image_path: String,
    pub description: String
}


#[rocket::async_trait]
impl<'r> FromData<'r> for PostInput {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;

        Outcome::Success(PostInput{ title: "".to_string(), body: "".to_string(), image_path: "".to_string(), description: "".to_string()})

    }
}