use serde::{Serialize, Deserialize};
use crate::entities::schemas::post_schema::PostSchema;
use crate::entities::io::output::error_entity::ErrorResponse;

#[derive(Serialize, Deserialize)]
pub struct PostOutput {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub image_path: String,
    pub description: String
}

impl From<PostSchema> for PostOutput {
    fn from(orig: PostSchema) -> Self {
        PostOutput {
            id: orig.id,
            title: orig.title,
            body: orig.body,
            image_path: orig.image_path,
            description: orig.description
        }
    }
}
