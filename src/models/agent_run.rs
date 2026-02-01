use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum AgentRole {
    Planner,
    Dev,
    QA,
    Reviewer,
}

impl std::fmt::Display for AgentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentRole::Planner => write!(f, "Planner"),
            AgentRole::Dev => write!(f, "Dev"),
            AgentRole::QA => write!(f, "QA"),
            AgentRole::Reviewer => write!(f, "Reviewer"),
        }
    }
}

impl std::str::FromStr for AgentRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Planner" => Ok(AgentRole::Planner),
            "Dev" => Ok(AgentRole::Dev),
            "QA" => Ok(AgentRole::QA),
            "Reviewer" => Ok(AgentRole::Reviewer),
            _ => Err(format!("Unknown agent role: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum AgentRunStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl std::fmt::Display for AgentRunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentRunStatus::Pending => write!(f, "Pending"),
            AgentRunStatus::Running => write!(f, "Running"),
            AgentRunStatus::Completed => write!(f, "Completed"),
            AgentRunStatus::Failed => write!(f, "Failed"),
        }
    }
}

impl std::str::FromStr for AgentRunStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(AgentRunStatus::Pending),
            "Running" => Ok(AgentRunStatus::Running),
            "Completed" => Ok(AgentRunStatus::Completed),
            "Failed" => Ok(AgentRunStatus::Failed),
            _ => Err(format!("Unknown agent run status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AgentRun {
    pub id: String,
    pub task_id: String,
    pub agent_role: String,
    pub status: String,
    pub runner_id: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub patch_content: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
}

impl AgentRun {
    pub fn role_enum(&self) -> AgentRole {
        self.agent_role.parse().unwrap_or(AgentRole::Dev)
    }

    pub fn status_enum(&self) -> AgentRunStatus {
        self.status.parse().unwrap_or(AgentRunStatus::Pending)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignAgentRequest {
    pub agent_role: AgentRole,
    #[serde(default)]
    pub priority: Option<i32>,
}

impl AgentRun {
    pub fn new(task_id: String, agent_role: AgentRole) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_id,
            agent_role: agent_role.to_string(),
            status: AgentRunStatus::Pending.to_string(),
            runner_id: None,
            started_at: None,
            completed_at: None,
            patch_content: None,
            error_message: None,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}
