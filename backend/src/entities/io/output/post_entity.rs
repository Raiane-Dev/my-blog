use serde::{Serialize, Deserialize};
use crate::entities::schemas::post_schema::PostSchema;
use crate::entities::io::output::error_entity::ErrorResponse;

#[derive(Serialize, Deserialize)]
pub struct PostOutput {
    pub title: String,
    pub body: String
}

impl From<PostSchema> for PostOutput {
    fn from(orig: PostSchema) -> Self {
        PostOutput {
            title: orig.title,
            body: orig.body,
        }
    }
}
