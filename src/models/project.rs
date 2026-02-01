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
    // Railway deployment fields
    pub railway_project_id: Option<String>,
    pub railway_service_id: Option<String>,
    pub railway_environment_id: Option<String>,
    pub deployment_url: Option<String>,
    pub deployment_status: String,
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
            railway_project_id: None,
            railway_service_id: None,
            railway_environment_id: None,
            deployment_url: None,
            deployment_status: "not_deployed".to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Extract "owner/repo" from GitHub URL
    pub fn github_repo(&self) -> Option<String> {
        let url = &self.github_url;
        // Handle both https://github.com/owner/repo and git@github.com:owner/repo
        if url.contains("github.com") {
            let parts: Vec<&str> = url.rsplitn(3, '/').collect();
            if parts.len() >= 2 {
                let repo = parts[0].trim_end_matches(".git");
                let owner = parts[1];
                return Some(format!("{}/{}", owner, repo));
            }
        }
        None
    }
}
