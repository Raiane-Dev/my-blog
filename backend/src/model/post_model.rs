use crate::entities::{io::input::post_entity::PostInput, schemas::post_schema::{self, PostSchema}};

use crate::config::database::POOL;

pub async fn create(data: PostInput) -> Result<i32, sqlx::Error> {
    let created_id: (i32, ) = sqlx::query_as!(
        r#"
            INSERT INTO posts (title, body)
            VALUES ($1, $2)
            RETURNING id
        "#,
        data.title, data.body,
    )
    .fetch_one(&*POOL)
    .await?;

    Ok(created_id.0)
}

pub async fn read_all() -> Result<Vec<PostSchema>, E> {

    let posts: Vec<PostSchema> = sqlx::query_as!(
        PostSchema,
        r#"
            SELECT * FROM posts
        "#,
    )
    .fetch_all(&*POOL)
    .await?;

    Ok(posts)
}