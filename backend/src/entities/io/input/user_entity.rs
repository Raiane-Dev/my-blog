use std::io::{self, Read};

use serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::request::{self, Request};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::outcome::Outcome;

#[derive(Debug)]
pub enum Error {
    InvalidUsername,
    InvalidEmail,
    InvalidPassword,
}

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    pub username: String, 
    pub email: String, 
    pub password: String,
}