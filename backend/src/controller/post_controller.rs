use rocket;
use rocket::response::status::NotFound;
use serde::Serialize;
use sqlx::types::Json;
use crate::entities::io::input::post_entity::PostInput;
use crate::entities::schemas::post_schema::PostSchema;

#[get("/posts")]
pub async fn get_posts() -> Result<Json<Vec<PostSchema>>, NotFound<String>> {
    let posts: Vec<PostSchema> = crate::model::post_model::read_all()
    .await
    .expect("Couldn't get posts");

    Ok(Json(posts))
}

#[post("/new-post", data = "<input>")]
pub fn new_post(input: PostInput) {

}