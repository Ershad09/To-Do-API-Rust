use axum::{response::Html, routing::get, Router};

use sqlx::PgPool;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
      
         .route("/", get(test))
        .with_state(pool)  
}
async fn test() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}