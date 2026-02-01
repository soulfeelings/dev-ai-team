use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use utoipa::{IntoParams, ToSchema};

use crate::error::AppResult;
use crate::models::{AgentRole, AgentRunStatus, CreateReasoningLogRequest, TaskStatus};
use crate::repositories::AgentRunRepository;
use crate::services::{AgentRunWithContext, AgentService, TaskQueue};

#[derive(Debug, Deserialize, IntoParams)]
pub struct PollQuery {
    pub runner_id: String,
    #[serde(default)]
    pub roles: Option<String>, // Comma-separated list: "Dev,QA"
}

/// Poll for available tasks
#[utoipa::path(
    get,
    path = "/api/runner/poll",
    params(PollQuery),
    responses(
        (status = 200, description = "Task found or empty", body = Option<AgentRunWithContext>)
    ),
    tag = "runner"
)]
pub async fn poll_task(
    State(pool): State<SqlitePool>,
    Query(query): Query<PollQuery>,
) -> AppResult<Json<Option<AgentRunWithContext>>> {
    let roles: Vec<AgentRole> = query
        .roles
        .as_deref()
        .unwrap_or("Dev,QA,Planner,Reviewer")
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let result = TaskQueue::poll_next(&pool, &query.runner_id, &roles).await?;
    Ok(Json(result))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct HeartbeatQuery {
    pub runner_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HeartbeatResponse {
    pub active_runs: u64,
}

/// Send heartbeat from runner
#[utoipa::path(
    post,
    path = "/api/runner/heartbeat",
    params(HeartbeatQuery),
    responses(
        (status = 200, description = "Heartbeat acknowledged", body = HeartbeatResponse)
    ),
    tag = "runner"
)]
pub async fn heartbeat(
    State(pool): State<SqlitePool>,
    Query(query): Query<HeartbeatQuery>,
) -> AppResult<Json<HeartbeatResponse>> {
    let active_runs = AgentRunRepository::update_heartbeat(&pool, &query.runner_id).await?;
    Ok(Json(HeartbeatResponse { active_runs }))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AgentResultRequest {
    pub agent_run_id: String,
    pub status: AgentRunStatus,
    pub patch_content: Option<String>,
    pub reasoning_log: Option<Vec<ReasoningLogEntry>>,
    pub chat_messages: Option<Vec<ChatMessageEntry>>,
    pub next_status: Option<TaskStatus>,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReasoningLogEntry {
    pub step: i32,
    pub thought: String,
    pub action: Option<String>,
    pub observation: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatMessageEntry {
    pub content: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AgentResultResponse {
    pub success: bool,
    pub message: String,
}

/// Submit agent result
#[utoipa::path(
    post,
    path = "/api/runner/result",
    request_body = AgentResultRequest,
    responses(
        (status = 200, description = "Result processed", body = AgentResultResponse)
    ),
    tag = "runner"
)]
pub async fn submit_result(
    State(pool): State<SqlitePool>,
    Json(req): Json<AgentResultRequest>,
) -> AppResult<Json<AgentResultResponse>> {
    let reasoning_logs: Vec<CreateReasoningLogRequest> = req
        .reasoning_log
        .unwrap_or_default()
        .into_iter()
        .map(|e| CreateReasoningLogRequest {
            step_number: e.step,
            thought: e.thought,
            action: e.action,
            observation: e.observation,
        })
        .collect();

    let chat_messages: Vec<String> = req
        .chat_messages
        .unwrap_or_default()
        .into_iter()
        .map(|e| e.content)
        .collect();

    AgentService::process_result(
        &pool,
        &req.agent_run_id,
        req.status,
        req.patch_content.as_deref(),
        req.error_message.as_deref(),
        reasoning_logs,
        chat_messages,
        req.next_status,
    )
    .await?;

    Ok(Json(AgentResultResponse {
        success: true,
        message: "Result processed successfully".to_string(),
    }))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct StreamLogRequest {
    pub agent_run_id: String,
    pub step: i32,
    pub thought: String,
    pub action: Option<String>,
    pub observation: Option<String>,
}

/// Stream a reasoning log entry
#[utoipa::path(
    post,
    path = "/api/runner/log",
    request_body = StreamLogRequest,
    responses(
        (status = 200, description = "Log entry saved")
    ),
    tag = "runner"
)]
pub async fn stream_log(
    State(pool): State<SqlitePool>,
    Json(req): Json<StreamLogRequest>,
) -> AppResult<Json<()>> {
    use crate::models::ReasoningLog;
    use crate::repositories::ReasoningLogRepository;

    let log = ReasoningLog::new(
        req.agent_run_id,
        CreateReasoningLogRequest {
            step_number: req.step,
            thought: req.thought,
            action: req.action,
            observation: req.observation,
        },
    );

    ReasoningLogRepository::create(&pool, &log).await?;
    Ok(Json(()))
}
