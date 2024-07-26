use serde::Serialize;
use rocket_okapi::okapi::openapi3::{
    Object, Responses, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};


#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct ErrorContent {
    pub code: u16,
    pub reason: String,
    pub description: Option<String>,
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct MyError {
    pub error: ErrorContent,
}
