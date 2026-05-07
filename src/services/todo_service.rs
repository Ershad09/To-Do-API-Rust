use crate::{
    models::todo::Todo,
    repositories::todo_repo::{create_todo, delete_todo, get_todo_by_id, update_todo},
};
use sqlx::PgPool;

pub async fn create_todo_service(pool: &PgPool, title: String) -> Result<Todo, String> {
    if title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }

    let todo = create_todo(pool, &title)
        .await
        .map_err(|_| "Database error".to_string())?;

    Ok(todo)
}

pub async fn get_todo_by_id_service(pool: &PgPool, id: i32) -> Result<Todo, String> {
    let todo = get_todo_by_id(pool, id)
        .await
        .map_err(|_| "Todo not found".to_string())?;

    Ok(todo)
}

pub async fn update_todo_service(
    pool: &PgPool,
    id: i32,
    title: String,
) -> Result<Todo, String> {

    if title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }

    let todo = update_todo(pool, id, &title)
        .await
        .map_err(|_| "Database error".to_string())?;

    Ok(todo)
}


pub async fn delete_todo_service(pool: &PgPool, id: i32) -> Result<(), String> {
    delete_todo(pool, id)
        .await
        .map_err(|_| "Database error".to_string())?;

    Ok(())
}


