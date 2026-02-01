use sqlx::SqlitePool;

use crate::error::{AppError, AppResult};
use crate::models::{
    AgentRole, AgentRun, AgentRunStatus, ChatMessage, CreateReasoningLogRequest, ReasoningLog,
    TaskStatus,
};
use crate::repositories::{AgentRunRepository, ChatRepository, ReasoningLogRepository, TaskRepository};

pub struct AgentService;

impl AgentService {
    /// Assign an agent to a task
    pub async fn assign_agent(
        pool: &SqlitePool,
        task_id: &str,
        role: AgentRole,
        priority: Option<i32>,
    ) -> AppResult<AgentRun> {
        // Verify task exists
        let task = TaskRepository::find_by_id(pool, task_id).await?;

        // Update task priority if specified
        if let Some(p) = priority {
            TaskRepository::update(
                pool,
                task_id,
                &crate::models::UpdateTaskRequest {
                    title: None,
                    description: None,
                    acceptance_criteria: None,
                    status: None,
                    priority: Some(p),
                    branch_name: None,
                },
            )
            .await?;
        }

        // Create agent run
        let agent_run = AgentRun::new(task.id, role);
        AgentRunRepository::create(pool, &agent_run).await?;

        Ok(agent_run)
    }

    /// Process agent result and update task status
    pub async fn process_result(
        pool: &SqlitePool,
        agent_run_id: &str,
        status: AgentRunStatus,
        patch_content: Option<&str>,
        error_message: Option<&str>,
        reasoning_logs: Vec<CreateReasoningLogRequest>,
        chat_messages: Vec<String>,
        next_task_status: Option<TaskStatus>,
    ) -> AppResult<AgentRun> {
        // Get the agent run
        let run = AgentRunRepository::find_by_id(pool, agent_run_id).await?;

        // Verify it's in Running state
        if run.status_enum() != AgentRunStatus::Running {
            return Err(AppError::BadRequest(format!(
                "AgentRun {} is not in Running state",
                agent_run_id
            )));
        }

        // Complete the run
        let completed_run =
            AgentRunRepository::complete_run(pool, agent_run_id, status, patch_content, error_message)
                .await?;

        // Get task for project_id
        let task = TaskRepository::find_by_id(pool, &run.task_id).await?;

        // Save reasoning logs
        if !reasoning_logs.is_empty() {
            let logs: Vec<ReasoningLog> = reasoning_logs
                .into_iter()
                .map(|req| ReasoningLog::new(agent_run_id.to_string(), req))
                .collect();
            ReasoningLogRepository::create_many(pool, &logs).await?;
        }

        // Save chat messages from agent
        if !chat_messages.is_empty() {
            let messages: Vec<ChatMessage> = chat_messages
                .into_iter()
                .map(|content| {
                    ChatMessage::from_agent(
                        task.project_id.clone(),
                        Some(task.id.clone()),
                        &run.agent_role,
                        content,
                    )
                })
                .collect();
            ChatRepository::create_many(pool, &messages).await?;
        }

        // Update task status if specified and run was successful
        if status == AgentRunStatus::Completed {
            if let Some(next_status) = next_task_status {
                TaskRepository::update_status(pool, &run.task_id, next_status).await?;
            }
        } else if status == AgentRunStatus::Failed {
            // Optionally mark task as failed
            TaskRepository::update_status(pool, &run.task_id, TaskStatus::Failed).await?;
        }

        Ok(completed_run)
    }

    /// Get the next task status based on current role
    pub fn get_next_task_status(current_status: TaskStatus, role: AgentRole) -> Option<TaskStatus> {
        match (current_status, role) {
            (TaskStatus::Backlog, AgentRole::Planner) => Some(TaskStatus::InProgress),
            (TaskStatus::InProgress, AgentRole::Dev) => Some(TaskStatus::QA),
            (TaskStatus::QA, AgentRole::QA) => Some(TaskStatus::Review),
            (TaskStatus::Review, AgentRole::Reviewer) => Some(TaskStatus::Done),
            _ => None,
        }
    }
}
