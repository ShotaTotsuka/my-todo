use std::sync::Arc;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::repositories::{CreateTodo, TodoRepository};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}
