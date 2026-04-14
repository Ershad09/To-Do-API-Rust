use crate::{ repositories::todo_repo::create_todo};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

pub async fn create_todo_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    if payload.title.trim().is_empty() {
        tracing::warn!("Create todo failed: title is empty");
        return StatusCode::BAD_REQUEST.into_response();
    }

    match create_todo(&pool, &payload.title).await {
        Ok(todo) => {
            tracing::info!("Todo created with id: {}", todo.id);
            (StatusCode::CREATED, Json(todo)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to create todo: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}