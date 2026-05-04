use crate::services::todo_service::{create_todo_service, get_todo_by_id_service};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
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
    tracing::info!("Received request to create todo");

    match create_todo_service(&pool, payload.title).await {
        Ok(todo) => {
            tracing::info!("Todo created with id: {}", todo.id);
            (StatusCode::CREATED, Json(todo)).into_response()
        }

        Err(e) => {
            tracing::warn!("Error creating todo: {}", e);

            if e == "Title cannot be empty" {
                tracing::info!("Validation failed: empty title");
                StatusCode::BAD_REQUEST.into_response()
            } else {
                tracing::error!("Internal server error while creating todo");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub async fn get_todo_by_id_handler(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    tracing::info!("Received request to fetch todo with id: {}", id);

    match get_todo_by_id_service(&pool, id).await {
        Ok(todo) => {
            tracing::info!("Todo found with id: {}", id);
            (StatusCode::OK, Json(todo)).into_response()
        }
        Err(e) => {
            tracing::warn!("Error fetching todo with id {}: {}", id, e);

            if e == "Todo not found" {
                tracing::info!("Todo not found for id: {}", id);
                StatusCode::NOT_FOUND.into_response()
            } else {
                tracing::error!("Internal server error while fetching todo with id: {}", id);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}




