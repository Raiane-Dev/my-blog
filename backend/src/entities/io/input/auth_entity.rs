use serde::{Serialize, Deserialize};
use std::io::{self, Read};
use rocket::http::Status;
use rocket::request::{self, Request};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::outcome::Outcome;

#[derive(Debug)]
pub enum Error {
    InvalidEmail,
    InvalidPassword,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    pub email: String,
    pub plain_password: String,
}