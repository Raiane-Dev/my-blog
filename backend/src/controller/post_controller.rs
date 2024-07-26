use rocket;
use rocket::{State, http::Status};
use rocket::serde::{Serialize, json::Json};

use rocket::response::status;
use crate::entities::io::input::post_entity::PostInput;
use crate::entities::io::output::post_entity::PostOutput;
use crate::entities::io::output::error_entity::ErrorResponse;
use crate::entities::schemas::post_schema::PostSchema;
use crate::entities::io::output::api_entity::{NetworkResponse, Response, ResponseBody};
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField};
use rocket::fs::{FileServer, relative};
use rocket::http::ContentType;
use rocket::Data;
use crate::utils::jwt::JWT;
use rocket::serde::json::serde_json;

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
pub async fn get_post(_id: &str) -> Result<status::Custom<Json<PostOutput>>, status::Custom<Json<ErrorResponse>>> {
    let post: PostSchema = crate::model::post_model::select_one("id = $1::integer".to_string(), vec![&_id])
    .await.unwrap();

    let post_output: PostOutput = post.into();

    Ok(status::Custom(Status::Ok, Json(post_output)))
}

#[post("/new-post", format = "application/json", data = "<input>")]
pub async fn new_post(_auth: JWT, input: Json<PostInput>) -> Result<String, NetworkResponse>
{
    match crate::model::post_model::create(input.into_inner())
    .await {
        Ok(_) => {
            let response = Response {body: ResponseBody::Message("success".to_string())};
            Ok(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            println!("{}", err);
            return Err(NetworkResponse::BadRequest(err.to_string()));
        }
    }
}

#[post("/upload", data = "<data>")]
pub async fn upload(_auth: JWT, content_type: &ContentType, data: Data<'_>) -> std::io::Result<String> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec![
            MultipartFormDataField::file("file").size_limit(20 * 1024 * 1024),
        ]
    );
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    if let Some(file) = multipart_form_data.files.get_mut("file") {
        let file_field = file.remove(0);
        let _file_name = file_field.file_name.unwrap_or_else(|| "file".to_string());
        let _file_content_type = file_field.content_type;
        let _path = file_field.path;
        tokio::fs::copy(&_path, format!("public/images/{}", _file_name)).await?;
    }

    Ok("File uploaded successfully".to_string())
}



#[delete("/post/<post_id>")]
pub async fn delete_post_handler(_auth: JWT, post_id: i32) -> Result<String, NetworkResponse> {
    match crate::model::post_model::delete_post(post_id).await {
        Ok(_result) => {
            let response = Response {body: ResponseBody::Message("success".to_string())};
            Ok(serde_json::to_string(&response).unwrap())
        },
        Err(err) => {
            println!("{}", err);
            return Err(NetworkResponse::BadRequest(err.to_string()));
        }
    }
}