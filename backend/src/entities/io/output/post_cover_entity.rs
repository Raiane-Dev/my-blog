use serde::{Serialize, Deserialize};
use crate::entities::schemas::post_cover_schema::PostCoverSchema;
use crate::entities::io::output::error_entity::ErrorResponse;

#[derive(Serialize, Deserialize)]
pub struct PostCoverOutput {
    pub post_id: i32,
    pub headline: String,
    pub description: String,
    pub image_path: String,
}

impl From<PostCoverSchema> for PostCoverOutput {
    fn from(orig: PostCoverSchema) -> Self {
        PostCoverOutput {
            post_id: orig.post_id,
            headline: orig.headline,
            description: orig.description,
            image_path: orig.image_path,
        }
    }
}
