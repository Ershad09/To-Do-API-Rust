use crate::models::todo::Todo;
use sqlx::PgPool;

pub async fn create_todo(pool: &PgPool, title: &str) -> Result<Todo, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title)
         VALUES ($1)
         RETURNING id, title, completed, created_at, updated_at"
    )
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}