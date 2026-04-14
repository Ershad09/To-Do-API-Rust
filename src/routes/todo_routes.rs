use axum::{Router, routing::post};
use sqlx::PgPool;
use crate::handlers::todo_handler::create_todo_handler;

pub fn todo_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", post(create_todo_handler))
        .with_state(pool)
}