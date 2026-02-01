use utoipa::OpenApi;

use crate::handlers::{chat, projects, runner, tasks};
use crate::models::{
    AgentRole, AgentRun, AgentRunStatus, AssignAgentRequest, ChatMessage, CreateChatMessageRequest,
    CreateProjectRequest, CreateTaskRequest, Project, ReasoningLog, Task, TaskStatus,
    UpdateProjectRequest, UpdateTaskRequest,
};
use crate::services::{AgentRunContext, AgentRunWithContext, QueueStats};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Projects
        projects::list_projects,
        projects::get_project,
        projects::create_project,
        projects::update_project,
        projects::delete_project,
        // Tasks
        tasks::list_tasks,
        tasks::create_task,
        tasks::get_task,
        tasks::update_task,
        tasks::delete_task,
        tasks::assign_agent,
        tasks::get_task_runs,
        // Runner
        runner::poll_task,
        runner::heartbeat,
        runner::submit_result,
        runner::stream_log,
        // Chat
        chat::get_project_chat,
        chat::send_chat_message,
        chat::get_task_chat,
    ),
    components(
        schemas(
            Project,
            CreateProjectRequest,
            UpdateProjectRequest,
            Task,
            TaskStatus,
            CreateTaskRequest,
            UpdateTaskRequest,
            AgentRole,
            AgentRunStatus,
            AgentRun,
            AssignAgentRequest,
            ReasoningLog,
            ChatMessage,
            CreateChatMessageRequest,
            AgentRunWithContext,
            AgentRunContext,
            QueueStats,
            runner::AgentResultRequest,
            runner::AgentResultResponse,
            runner::ReasoningLogEntry,
            runner::ChatMessageEntry,
            runner::HeartbeatResponse,
            runner::StreamLogRequest,
        )
    ),
    tags(
        (name = "projects", description = "Project management endpoints"),
        (name = "tasks", description = "Task management endpoints"),
        (name = "runner", description = "Runner polling and result submission"),
        (name = "chat", description = "Team chat endpoints"),
    ),
    info(
        title = "AI Dev Workspace API",
        version = "0.1.0",
        description = "Backend API for managing AI agents that work on tasks from GitHub repositories"
    )
)]
pub struct ApiDoc;
