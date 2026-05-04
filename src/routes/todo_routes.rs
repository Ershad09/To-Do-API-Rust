use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use crate::handlers::todo_handler::{create_todo_handler, get_todo_by_id_handler};

pub fn todo_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", post(create_todo_handler))
        .route("/todos/{id}", get(get_todo_by_id_handler))
        .with_state(pool)
}


