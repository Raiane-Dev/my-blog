use sqlx::postgres::PgRow;

use crate::entities::{io::input::post_entity::PostInput, schemas::post_schema::{self, PostSchema}};
use std::error::Error;
use crate::config::database::INSTANCE;

pub async fn create(data: PostInput) -> Result<PgRow, sqlx::Error> {
    let created_id = sqlx::query(
        r#"
            INSERT INTO posts (title, body)
            VALUES ($1, $2)
            RETURNING id
        "#,
    )
    .bind(data.title)
    .bind(data.body)
    .fetch_one(&*INSTANCE)
    .await?;

    Ok(created_id)
}

pub async fn read_all() -> Result<Vec<PostSchema>, sqlx::Error> {

    let query = sqlx::query_as!(
        PostSchema,
        "SELECT * FROM posts",
    )
    .fetch_all(&*INSTANCE)
    .await;

    return query;
}