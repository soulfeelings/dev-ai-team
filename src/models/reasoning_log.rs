use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ReasoningLog {
    pub id: String,
    pub agent_run_id: String,
    pub step_number: i32,
    pub thought: String,
    pub action: Option<String>,
    pub observation: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReasoningLogRequest {
    pub step_number: i32,
    pub thought: String,
    pub action: Option<String>,
    pub observation: Option<String>,
}

impl ReasoningLog {
    pub fn new(agent_run_id: String, req: CreateReasoningLogRequest) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            agent_run_id,
            step_number: req.step_number,
            thought: req.thought,
            action: req.action,
            observation: req.observation,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReasoningLogEntry {
    pub step: i32,
    pub thought: String,
    pub action: Option<String>,
    pub observation: Option<String>,
}
