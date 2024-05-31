use rocket;
use rocket::{State, http::Status};
use rocket::serde::{Serialize, json::Json};

use rocket::response::status;
use crate::entities::io::input::post_entity::PostInput;
use crate::entities::io::output::post_cover_entity::PostCoverOutput;
use crate::entities::io::output::error_entity::ErrorResponse;
use crate::entities::schemas::post_cover_schema::PostCoverSchema;
use crate::entities::io::output::api_entity::{NetworkResponse, Response, ResponseBody};


#[get("/covers")]
pub async fn get_cover_posts() -> Result<status::Custom<Json<Vec<PostCoverOutput>>>, status::Custom<Json<ErrorResponse>>> {
    let posts: Vec<PostCoverSchema> = crate::model::post_cover_model::select_all()
    .await.unwrap();

    let post_output: Vec<PostCoverOutput> = posts.into_iter().map(PostCoverOutput::from).collect();

    if post_output.is_empty() {
        return Err(
            status::Custom(
                Status::BadRequest, 
                Json(ErrorResponse { 
                    message: "Empty".to_string()
                }
            )))
    }

    Ok(status::Custom(Status::Ok, Json(post_output)))
}