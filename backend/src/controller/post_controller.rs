use rocket;

use crate::entities::post_entity::PostInput as PostInput;

#[get("/posts")]
fn get_posts() -> &'static str {
    "hello"
}

#[post("/new-post", data = "<input>")]
fn new_post(input: PostInput) {

}