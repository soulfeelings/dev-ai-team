use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub github_url: String,
    pub local_path: Option<String>,
    pub default_branch: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectRequest {
    pub name: String,
    pub github_url: String,
    pub local_path: Option<String>,
    #[serde(default = "default_branch")]
    pub default_branch: String,
}

fn default_branch() -> String {
    "main".to_string()
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub github_url: Option<String>,
    pub local_path: Option<String>,
    pub default_branch: Option<String>,
}

impl Project {
    pub fn new(req: CreateProjectRequest) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name: req.name,
            github_url: req.github_url,
            local_path: req.local_path,
            default_branch: req.default_branch,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
