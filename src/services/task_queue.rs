use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use utoipa::ToSchema;

use crate::error::AppResult;
use crate::models::{AgentRole, AgentRun, ChatMessage, Project, Task};
use crate::repositories::{AgentRunRepository, ChatRepository, ProjectRepository, TaskRepository};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AgentRunContext {
    pub branch_name: Option<String>,
    pub previous_runs: Vec<AgentRun>,
    pub related_chat: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AgentRunWithContext {
    pub agent_run: AgentRun,
    pub task: Task,
    pub project: Project,
    pub context: AgentRunContext,
}

pub struct TaskQueue;

impl TaskQueue {
    /// Poll for the next available task for the given runner and roles
    pub async fn poll_next(
        pool: &SqlitePool,
        runner_id: &str,
        roles: &[AgentRole],
    ) -> AppResult<Option<AgentRunWithContext>> {
        // Find pending runs for the specified roles
        let pending_runs = AgentRunRepository::find_pending_by_roles(pool, roles).await?;

        if pending_runs.is_empty() {
            return Ok(None);
        }

        // Try to claim the first available run
        let run = &pending_runs[0];
        let claimed_run = AgentRunRepository::claim_run(pool, &run.id, runner_id).await?;

        // Fetch task and project
        let task = TaskRepository::find_by_id(pool, &claimed_run.task_id).await?;
        let project = ProjectRepository::find_by_id(pool, &task.project_id).await?;

        // Fetch context
        let previous_runs = AgentRunRepository::find_by_task(pool, &task.id).await?;
        let related_chat = ChatRepository::find_by_task(pool, &task.id, Some(50)).await?;

        let context = AgentRunContext {
            branch_name: task.branch_name.clone(),
            previous_runs,
            related_chat,
        };

        Ok(Some(AgentRunWithContext {
            agent_run: claimed_run,
            task,
            project,
            context,
        }))
    }

    /// Get queue statistics
    pub async fn get_stats(pool: &SqlitePool) -> AppResult<QueueStats> {
        let pending: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM agent_runs WHERE status = 'Pending'")
                .fetch_one(pool)
                .await?;

        let running: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM agent_runs WHERE status = 'Running'")
                .fetch_one(pool)
                .await?;

        let completed: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM agent_runs WHERE status = 'Completed'")
                .fetch_one(pool)
                .await?;

        let failed: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM agent_runs WHERE status = 'Failed'")
                .fetch_one(pool)
                .await?;

        Ok(QueueStats {
            pending: pending.0 as u64,
            running: running.0 as u64,
            completed: completed.0 as u64,
            failed: failed.0 as u64,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct QueueStats {
    pub pending: u64,
    pub running: u64,
    pub completed: u64,
    pub failed: u64,
}
