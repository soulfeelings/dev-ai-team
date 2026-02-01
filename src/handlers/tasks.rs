use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;

use crate::error::AppResult;
use crate::models::{AgentRun, AssignAgentRequest, CreateTaskRequest, Task, UpdateTaskRequest};
use crate::repositories::{AgentRunRepository, TaskRepository};
use crate::services::AgentService;

/// List all tasks for a project
#[utoipa::path(
    get,
    path = "/api/projects/{project_id}/tasks",
    params(
        ("project_id" = String, Path, description = "Project ID")
    ),
    responses(
        (status = 200, description = "List of tasks", body = Vec<Task>)
    ),
    tag = "tasks"
)]
pub async fn list_tasks(
    State(pool): State<SqlitePool>,
    Path(project_id): Path<String>,
) -> AppResult<Json<Vec<Task>>> {
    let tasks = TaskRepository::find_all_by_project(&pool, &project_id).await?;
    Ok(Json(tasks))
}

/// Create a new task
#[utoipa::path(
    post,
    path = "/api/projects/{project_id}/tasks",
    params(
        ("project_id" = String, Path, description = "Project ID")
    ),
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created", body = Task)
    ),
    tag = "tasks"
)]
pub async fn create_task(
    State(pool): State<SqlitePool>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateTaskRequest>,
) -> AppResult<Json<Task>> {
    let task = Task::new(project_id, req);
    TaskRepository::create(&pool, &task).await?;
    Ok(Json(task))
}

/// Get a task by ID
#[utoipa::path(
    get,
    path = "/api/tasks/{id}",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task found", body = Task),
        (status = 404, description = "Task not found")
    ),
    tag = "tasks"
)]
pub async fn get_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> AppResult<Json<Task>> {
    let task = TaskRepository::find_by_id(&pool, &id).await?;
    Ok(Json(task))
}

/// Update a task
#[utoipa::path(
    patch,
    path = "/api/tasks/{id}",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    request_body = UpdateTaskRequest,
    responses(
        (status = 200, description = "Task updated", body = Task),
        (status = 404, description = "Task not found")
    ),
    tag = "tasks"
)]
pub async fn update_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTaskRequest>,
) -> AppResult<Json<Task>> {
    let task = TaskRepository::update(&pool, &id, &req).await?;
    Ok(Json(task))
}

/// Delete a task
#[utoipa::path(
    delete,
    path = "/api/tasks/{id}",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 204, description = "Task deleted"),
        (status = 404, description = "Task not found")
    ),
    tag = "tasks"
)]
pub async fn delete_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> AppResult<()> {
    TaskRepository::delete(&pool, &id).await?;
    Ok(())
}

/// Assign an agent to a task
#[utoipa::path(
    post,
    path = "/api/tasks/{id}/assign-agent",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    request_body = AssignAgentRequest,
    responses(
        (status = 200, description = "Agent assigned", body = AgentRun),
        (status = 404, description = "Task not found")
    ),
    tag = "tasks"
)]
pub async fn assign_agent(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(req): Json<AssignAgentRequest>,
) -> AppResult<Json<AgentRun>> {
    let run = AgentService::assign_agent(&pool, &id, req.agent_role, req.priority).await?;
    Ok(Json(run))
}

/// Get agent run history for a task
#[utoipa::path(
    get,
    path = "/api/tasks/{id}/runs",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "List of agent runs", body = Vec<AgentRun>)
    ),
    tag = "tasks"
)]
pub async fn get_task_runs(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> AppResult<Json<Vec<AgentRun>>> {
    let runs = AgentRunRepository::find_by_task(&pool, &id).await?;
    Ok(Json(runs))
}
