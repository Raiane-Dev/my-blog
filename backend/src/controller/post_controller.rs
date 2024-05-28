use rocket;
use rocket::{State, http::Status};
use rocket::serde::{Serialize, json::Json};
use rocket::response::status;

use crate::entities::io::input::post_entity::PostInput;
use crate::entities::io::output::post_entity::PostOutput;
use crate::entities::io::output::error_entity::ErrorResponse;
use crate::entities::schemas::post_schema::PostSchema;


// pub async fn get_by_id(user_service: &State<Box<dyn UserServiceTrait>>, id: &str) -> Result<status::Custom<Json<GetUserResponse>>, status::Custom<Json<ErrorResponse>>> {

#[get("/posts")]
pub async fn get_posts() -> status::Custom<Json<Vec<PostOutput>>> {
    let posts: Vec<PostSchema> = crate::model::post_model::read_all()
    .await.unwrap();

    let post_output: Vec<PostOutput> = posts.into_iter().map(PostOutput::from).collect();

    if post_output.is_empty() {
        // return Err(status::Custom(Status::InternalServerError, Json(ErrorResponse { message: "Error".to_string()})))
    }

    status::Custom(Status::Ok, Json(post_output))
}

#[post("/new-post", data = "<input>")]
pub fn new_post(input: PostInput) {

}