mod app;

mod config;
use config::settings::Settings;

mod database;
use database::db::create_db_pool;

mod models;

mod repositories;

mod errors;

mod handlers;
mod routes;

mod services;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let settings = Settings::default();
    tracing::info!("Setting loaded");

    let pool = create_db_pool(&settings.database_url, settings.db_max_connections).await;

    tracing::info!("Database connected!");

    let app = app::create_app(pool);
    tracing::info!("App created!");

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", settings.port))
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
