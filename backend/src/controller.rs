use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::service::TodoService;
use crate::models::Todo;

#[derive(Clone)]
pub struct AppState<T: TodoService>{
    pub todo_service: Arc<T>,
    }

pub fn create_todo_router<T: TodoService + Send + Sync + 'static + Clone>(todo_service: T) -> Router{
    let state = AppState {
        todo_service: Arc::new(todo_service),
        };

    Router::new()
    .route("/todos", get(get_all_todos::<T>))
    .route("/todos/{id}", get(get_todo_by_id::<T>)).with_state(state)
    }

#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
    description: String,
}

#[derive(Serialize)]
struct TodoResponse {
    id: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            completed: todo.completed,
        }
    }
}

async fn get_all_todos<T: TodoService>(
    State(state): State<AppState<T>>,
) -> impl IntoResponse {
    match state.todo_service.get_all_todos().await {
        Ok(todos) => Json(todos.into_iter().map(TodoResponse::from).collect::<Vec<_>>()).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch todos").into_response(),
    }
}

async fn get_todo_by_id<T: TodoService>(
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
     println!("🟡 get_todo_by_id called with id = {}", id); 
    match state.todo_service.get_todo_by_id(id).await {
        Ok(Some(todo)) => Json(TodoResponse::from(todo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Todo not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch todo").into_response(),
    }
}
