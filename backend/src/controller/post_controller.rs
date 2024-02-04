use rocket;

use crate::entities::io::input::post_entity::PostInput as PostInput;

#[get("/posts")]
pub fn get_posts() -> &'static str {
    "hello"
}

#[post("/new-post", data = "<input>")]
pub fn new_post(input: PostInput) {

}