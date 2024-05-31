use rocket;
use rocket::{State, http::Status};
use rocket::serde::{Serialize, json::Json};

use rocket::response::status;
use crate::entities::io::input::post_entity::PostInput;
use crate::entities::io::output::post_entity::PostOutput;
use crate::entities::io::output::error_entity::ErrorResponse;
use crate::entities::schemas::post_schema::PostSchema;
use crate::entities::io::output::api_entity::{NetworkResponse, Response, ResponseBody};


#[get("/posts")]
pub async fn get_posts() -> Result<status::Custom<Json<Vec<PostOutput>>>, status::Custom<Json<ErrorResponse>>> {
    let posts: Vec<PostSchema> = crate::model::post_model::select_all()
    .await.unwrap();

    let post_output: Vec<PostOutput> = posts.into_iter().map(PostOutput::from).collect();

    if post_output.is_empty() {
        return Err(
            status::Custom(
                Status::InternalServerError, 
                Json(ErrorResponse { 
                    message: "Empty".to_string()
                }
            )))
    }

    Ok(status::Custom(Status::Ok, Json(post_output)))
}

#[get("/post/<_id>")]
pub async fn get_post(_id: String) -> Result<status::Custom<Json<PostOutput>>, status::Custom<Json<ErrorResponse>>> {
    let post: PostSchema = crate::model::post_model::select_one("id = $1::integer".to_string(), vec![&_id])
    .await.unwrap();

    let post_output: PostOutput = post.into();

    Ok(status::Custom(Status::Ok, Json(post_output)))
}

#[post("/new-post", format = "application/json", data = "<input>")]
pub fn new_post(input: Json<PostInput>) {

}
