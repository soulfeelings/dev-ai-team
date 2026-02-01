use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;

use crate::error::AppResult;
use crate::models::{CreateProjectRequest, Project, UpdateProjectRequest};
use crate::repositories::ProjectRepository;

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
