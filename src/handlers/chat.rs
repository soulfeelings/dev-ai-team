use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use utoipa::IntoParams;

use crate::error::AppResult;
use crate::models::{ChatMessage, CreateChatMessageRequest};
use crate::repositories::ChatRepository;

#[derive(Debug, Deserialize, IntoParams)]
pub struct ChatQuery {
    #[serde(default)]
    pub limit: Option<i64>,
}

/// Get chat history for a project
#[utoipa::path(
    get,
    path = "/api/projects/{project_id}/chat",
    params(
        ("project_id" = String, Path, description = "Project ID"),
        ChatQuery
    ),
    responses(
        (status = 200, description = "Chat messages", body = Vec<ChatMessage>)
    ),
    tag = "chat"
)]
pub async fn get_project_chat(
    State(pool): State<SqlitePool>,
    Path(project_id): Path<String>,
    Query(query): Query<ChatQuery>,
) -> AppResult<Json<Vec<ChatMessage>>> {
    let messages = ChatRepository::find_by_project(&pool, &project_id, query.limit).await?;
    Ok(Json(messages))
}

/// Send a message to project chat
#[utoipa::path(
    post,
    path = "/api/projects/{project_id}/chat",
    params(
        ("project_id" = String, Path, description = "Project ID")
    ),
    request_body = CreateChatMessageRequest,
    responses(
        (status = 201, description = "Message sent", body = ChatMessage)
    ),
    tag = "chat"
)]
pub async fn send_chat_message(
    State(pool): State<SqlitePool>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateChatMessageRequest>,
) -> AppResult<Json<ChatMessage>> {
    let message = ChatMessage::new(project_id, req);
    ChatRepository::create(&pool, &message).await?;
    Ok(Json(message))
}

/// Get chat history for a task
#[utoipa::path(
    get,
    path = "/api/tasks/{task_id}/chat",
    params(
        ("task_id" = String, Path, description = "Task ID"),
        ChatQuery
    ),
    responses(
        (status = 200, description = "Chat messages", body = Vec<ChatMessage>)
    ),
    tag = "chat"
)]
pub async fn get_task_chat(
    State(pool): State<SqlitePool>,
    Path(task_id): Path<String>,
    Query(query): Query<ChatQuery>,
) -> AppResult<Json<Vec<ChatMessage>>> {
    let messages = ChatRepository::find_by_task(&pool, &task_id, query.limit).await?;
    Ok(Json(messages))
}
