use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum TaskStatus {
    Backlog,
    InProgress,
    QA,
    Review,
    Done,
    Failed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Backlog => write!(f, "Backlog"),
            TaskStatus::InProgress => write!(f, "InProgress"),
            TaskStatus::QA => write!(f, "QA"),
            TaskStatus::Review => write!(f, "Review"),
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Failed => write!(f, "Failed"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Backlog" => Ok(TaskStatus::Backlog),
            "InProgress" => Ok(TaskStatus::InProgress),
            "QA" => Ok(TaskStatus::QA),
            "Review" => Ok(TaskStatus::Review),
            "Done" => Ok(TaskStatus::Done),
            "Failed" => Ok(TaskStatus::Failed),
            _ => Err(format!("Unknown task status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub acceptance_criteria: String,
    pub status: String,
    pub priority: i32,
    pub parent_task_id: Option<String>,
    pub branch_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Task {
    pub fn status_enum(&self) -> TaskStatus {
        self.status.parse().unwrap_or(TaskStatus::Backlog)
    }
}

#[derive(Debug, Deserialize, ToSchema, Default)]
pub struct CreateTaskRequest {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub acceptance_criteria: Option<String>,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub parent_task_id: Option<String>,
    #[serde(default)]
    pub branch_name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub acceptance_criteria: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<i32>,
    pub branch_name: Option<String>,
}

impl Task {
    pub fn new(project_id: String, req: CreateTaskRequest) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            project_id,
            title: req.title,
            description: req.description.unwrap_or_default(),
            acceptance_criteria: req.acceptance_criteria.unwrap_or_default(),
            status: TaskStatus::Backlog.to_string(),
            priority: req.priority,
            parent_task_id: req.parent_task_id,
            branch_name: req.branch_name,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
