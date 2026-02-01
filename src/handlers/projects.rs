use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::env;
use utoipa::ToSchema;

use crate::error::{AppError, AppResult};
use crate::models::{CreateProjectRequest, Project, UpdateProjectRequest};
use crate::repositories::ProjectRepository;
use crate::services::RailwayService;

/// List all projects
#[utoipa::path(
    get,
    path = "/api/projects",
    responses(
        (status = 200, description = "List of projects", body = Vec<Project>)
    ),
    tag = "projects"
)]
pub async fn list_projects(State(pool): State<SqlitePool>) -> AppResult<Json<Vec<Project>>> {
    let projects = ProjectRepository::find_all(&pool).await?;
    Ok(Json(projects))
}

/// Get a project by ID
#[utoipa::path(
    get,
    path = "/api/projects/{id}",
    params(
        ("id" = String, Path, description = "Project ID")
    ),
    responses(
        (status = 200, description = "Project found", body = Project),
        (status = 404, description = "Project not found")
    ),
    tag = "projects"
)]
pub async fn get_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> AppResult<Json<Project>> {
    let project = ProjectRepository::find_by_id(&pool, &id).await?;
    Ok(Json(project))
}

/// Create a new project
#[utoipa::path(
    post,
    path = "/api/projects",
    request_body = CreateProjectRequest,
    responses(
        (status = 201, description = "Project created", body = Project)
    ),
    tag = "projects"
)]
pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreateProjectRequest>,
) -> AppResult<Json<Project>> {
    let project = Project::new(req);
    ProjectRepository::create(&pool, &project).await?;
    Ok(Json(project))
}

/// Update a project
#[utoipa::path(
    patch,
    path = "/api/projects/{id}",
    params(
        ("id" = String, Path, description = "Project ID")
    ),
    request_body = UpdateProjectRequest,
    responses(
        (status = 200, description = "Project updated", body = Project),
        (status = 404, description = "Project not found")
    ),
    tag = "projects"
)]
pub async fn update_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProjectRequest>,
) -> AppResult<Json<Project>> {
    let project = ProjectRepository::update(&pool, &id, &req).await?;
    Ok(Json(project))
}

/// Delete a project
#[utoipa::path(
    delete,
    path = "/api/projects/{id}",
    params(
        ("id" = String, Path, description = "Project ID")
    ),
    responses(
        (status = 204, description = "Project deleted"),
        (status = 404, description = "Project not found")
    ),
    tag = "projects"
)]
pub async fn delete_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> AppResult<()> {
    ProjectRepository::delete(&pool, &id).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeployResponse {
    pub railway_project_id: String,
    pub railway_service_id: String,
    pub deployment_status: String,
    pub message: String,
}

/// Deploy project to Railway
#[utoipa::path(
    post,
    path = "/api/projects/{id}/deploy",
    params(
        ("id" = String, Path, description = "Project ID")
    ),
    responses(
        (status = 200, description = "Deployment started", body = DeployResponse),
        (status = 404, description = "Project not found"),
        (status = 400, description = "Railway not configured or invalid GitHub URL")
    ),
    tag = "projects"
)]
pub async fn deploy_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> AppResult<Json<DeployResponse>> {
    let project = ProjectRepository::find_by_id(&pool, &id).await?;

    // Check if Railway API token is configured
    let api_token = env::var("RAILWAY_API_TOKEN")
        .map_err(|_| AppError::BadRequest("RAILWAY_API_TOKEN not configured".to_string()))?;

    // Extract owner/repo from GitHub URL
    let github_repo = project
        .github_repo()
        .ok_or_else(|| AppError::BadRequest("Invalid GitHub URL format".to_string()))?;

    let railway = RailwayService::new(api_token);

    // Check if already deployed - if so, trigger redeploy
    if let (Some(service_id), Some(env_id)) = (&project.railway_service_id, &project.railway_environment_id) {
        railway.redeploy(service_id, env_id).await?;

        ProjectRepository::update_deployment_status(&pool, &id, project.deployment_url.as_deref(), "deploying").await?;

        return Ok(Json(DeployResponse {
            railway_project_id: project.railway_project_id.unwrap_or_default(),
            railway_service_id: service_id.clone(),
            deployment_status: "deploying".to_string(),
            message: "Redeployment triggered".to_string(),
        }));
    }

    // Create new Railway project and service
    let deployment = railway
        .deploy_from_github(&project.name, &github_repo, &project.default_branch)
        .await?;

    // Save deployment info to database
    ProjectRepository::update_deployment(
        &pool,
        &id,
        &deployment.railway_project_id,
        &deployment.railway_service_id,
        &deployment.railway_environment_id,
        deployment.deployment_url.as_deref(),
        &deployment.status,
    )
    .await?;

    Ok(Json(DeployResponse {
        railway_project_id: deployment.railway_project_id,
        railway_service_id: deployment.railway_service_id,
        deployment_status: deployment.status,
        message: "Deployment started".to_string(),
    }))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeploymentStatusResponse {
    pub status: String,
    pub deployment_url: Option<String>,
}

/// Get deployment status
#[utoipa::path(
    get,
    path = "/api/projects/{id}/deployment",
    params(
        ("id" = String, Path, description = "Project ID")
    ),
    responses(
        (status = 200, description = "Deployment status", body = DeploymentStatusResponse),
        (status = 404, description = "Project not found or not deployed")
    ),
    tag = "projects"
)]
pub async fn get_deployment_status(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> AppResult<Json<DeploymentStatusResponse>> {
    let project = ProjectRepository::find_by_id(&pool, &id).await?;

    let (project_id, service_id, env_id) = match (
        &project.railway_project_id,
        &project.railway_service_id,
        &project.railway_environment_id,
    ) {
        (Some(p), Some(s), Some(e)) => (p, s, e),
        _ => {
            return Ok(Json(DeploymentStatusResponse {
                status: "not_deployed".to_string(),
                deployment_url: None,
            }));
        }
    };

    // Check if Railway API token is configured
    let api_token = match env::var("RAILWAY_API_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            return Ok(Json(DeploymentStatusResponse {
                status: project.deployment_status,
                deployment_url: project.deployment_url,
            }));
        }
    };

    let railway = RailwayService::new(api_token);

    match railway.get_deployment_status(project_id, service_id, env_id).await {
        Ok(deployment) => {
            // Update status in database if changed
            let url = deployment.static_url.as_ref().map(|u| format!("https://{}", u));
            if deployment.status != project.deployment_status || url != project.deployment_url {
                ProjectRepository::update_deployment_status(
                    &pool,
                    &id,
                    url.as_deref(),
                    &deployment.status,
                )
                .await?;
            }

            Ok(Json(DeploymentStatusResponse {
                status: deployment.status,
                deployment_url: url,
            }))
        }
        Err(_) => Ok(Json(DeploymentStatusResponse {
            status: project.deployment_status,
            deployment_url: project.deployment_url,
        })),
    }
}
