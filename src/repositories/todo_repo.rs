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


pub async fn get_todo_by_id(pool: &PgPool, id: i32) -> Result<Todo, sqlx::Error>{
    let todo = sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}


pub async fn update_todo(pool: &PgPool, id: i32, title: &str) -> Result<Todo, sqlx::Error>{
        let todo = sqlx::query_as::<_, Todo>(
             "UPDATE todos SET title = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 RETURNING *"
        )
        .bind(title)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(todo)
}


pub async fn delete_todo(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}