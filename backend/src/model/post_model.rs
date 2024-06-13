use sqlx::postgres::PgRow;

use crate::entities::{io::input::post_entity::PostInput, schemas::post_schema::{self, PostSchema}};
use std::error::Error;
use crate::config::database::INSTANCE;
use crate::config::database::get_db_pool;

pub async fn create(data: PostInput) -> Result<PgRow, sqlx::Error> {
    let pool = get_db_pool().await;

    let created_id = sqlx::query(
        r#"
            INSERT INTO posts (title, body, image_path, description)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#,
    )
    .bind(data.title)
    .bind(data.body)
    .bind(data.image_path)
    .bind(data.description)
    .fetch_one(pool)
    .await?;

    Ok(created_id)
}

pub async fn select_all() -> Result<Vec<PostSchema>, sqlx::Error> {

    let pool = get_db_pool().await;

    let query = sqlx::query_as!(
        PostSchema,
        "SELECT * FROM posts",
    )
    .fetch_all(pool)
    .await;

    return query;
}

pub async fn select_one(condition: String, args: Vec<&str>) -> Result<PostSchema, sqlx::Error> {

    let pool = get_db_pool().await;

    let query_str = format!("SELECT * FROM posts WHERE {}", condition);

    let mut query = sqlx::query_as::<_, PostSchema>(&query_str);

    for arg in args {
        query = query.bind(arg);
    }

    let result = query.fetch_one(pool).await;

    result
}
