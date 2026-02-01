use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ChatMessage {
    pub id: String,
    pub task_id: Option<String>,
    pub project_id: String,
    pub sender_type: String,
    pub sender_name: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateChatMessageRequest {
    pub task_id: Option<String>,
    pub sender_type: String,
    pub sender_name: String,
    pub content: String,
}

impl ChatMessage {
    pub fn new(project_id: String, req: CreateChatMessageRequest) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_id: req.task_id,
            project_id,
            sender_type: req.sender_type,
            sender_name: req.sender_name,
            content: req.content,
            created_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn from_agent(
        project_id: String,
        task_id: Option<String>,
        agent_role: &str,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_id,
            project_id,
            sender_type: agent_role.to_lowercase(),
            sender_name: format!("Agent ({})", agent_role),
            content,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}
