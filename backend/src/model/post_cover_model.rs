use sqlx::postgres::PgRow;

use crate::entities::{io::input::post_cover_entity::PostCoverInput, schemas::post_cover_schema::{self, PostCoverSchema}};
use std::error::Error;
use crate::config::database::INSTANCE;
use crate::config::database::get_db_pool;

pub async fn create(data: PostCoverInput) -> Result<PgRow, sqlx::Error> {
    let pool = get_db_pool().await;

    let created_id = sqlx::query(
        r#"
            INSERT INTO post_cover (post_id, headline, description, image_path)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#,
    )
    .bind(data.post_id)
    .bind(data.headline)
    .bind(data.description)
    .bind(data.image_path)
    .fetch_one(pool)
    .await?;

    Ok(created_id)
}

pub async fn select_all() -> Result<Vec<PostCoverSchema>, sqlx::Error> {

    let pool = get_db_pool().await;

    let query = sqlx::query_as!(
        PostCoverSchema,
        "SELECT * FROM post_cover",
    )
    .fetch_all(pool)
    .await;

    return query;
}

pub async fn select_one(condition: String, args: Vec<&str>) -> Result<PostCoverSchema, sqlx::Error> {

    let pool = get_db_pool().await;

    let query_str = format!("SELECT * FROM post_cover WHERE {}", condition);

    let mut query = sqlx::query_as::<_, PostCoverSchema>(&query_str);

    for arg in args {
        query = query.bind(arg);
    }

    let result = query.fetch_one(pool).await;

    result
}
