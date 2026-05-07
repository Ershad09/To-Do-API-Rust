use axum::{Router, routing::{get, post, put, delete}};
use sqlx::PgPool;
use crate::handlers::todo_handler::{create_todo_handler, delete_todo_handler, get_todo_by_id_handler, update_todo_handler};

pub fn todo_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", post(create_todo_handler))
        .route("/todos/{id}", get(get_todo_by_id_handler))
        .route("/todos/:id", put(update_todo_handler))
        .route("/todos/:id", delete(delete_todo_handler))
        .with_state(pool)
}


