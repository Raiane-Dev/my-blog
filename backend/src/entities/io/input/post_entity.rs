use serde::Serialize;

#[derive(Serialize)] 
pub struct GenecticResponse {
    pub status: String,
    pub messages: String,
}

pub struct PostInput {
    pub title: String, 
    pub body: String,
}