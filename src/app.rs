use axum::Router;

use sqlx::PgPool;

use crate::routes::todo_routes::todo_routes;

pub fn create_app(pool: PgPool) -> Router {
    Router::new().merge(todo_routes(pool))
}
