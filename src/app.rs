use axum::{response::Html, routing::get, Router};

pub fn create_app() -> Router {
    Router::new().route("/", get(test))
}

async fn test() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}