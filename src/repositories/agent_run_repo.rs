use chrono::Utc;
use sqlx::SqlitePool;

use crate::error::{AppError, AppResult};
use crate::models::{AgentRole, AgentRun, AgentRunStatus, ReasoningLog};

pub struct AgentRunRepository;

impl AgentRunRepository {
    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> AppResult<AgentRun> {
        sqlx::query_as::<_, AgentRun>("SELECT * FROM agent_runs WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("AgentRun not found: {}", id)))
    }

    pub async fn find_by_task(pool: &SqlitePool, task_id: &str) -> AppResult<Vec<AgentRun>> {
        let runs = sqlx::query_as::<_, AgentRun>(
            "SELECT * FROM agent_runs WHERE task_id = ? ORDER BY created_at DESC",
        )
        .bind(task_id)
        .fetch_all(pool)
        .await?;

        Ok(runs)
    }

    pub async fn find_pending_by_roles(
        pool: &SqlitePool,
        roles: &[AgentRole],
    ) -> AppResult<Vec<AgentRun>> {
        if roles.is_empty() {
            return Ok(vec![]);
        }

        // Build the IN clause dynamically
        let placeholders: Vec<&str> = roles.iter().map(|_| "?").collect();
        let query = format!(
            r#"
            SELECT ar.* FROM agent_runs ar
            JOIN tasks t ON ar.task_id = t.id
            WHERE ar.status = 'Pending'
              AND ar.agent_role IN ({})
            ORDER BY t.priority DESC, ar.created_at ASC
            "#,
            placeholders.join(", ")
        );

        let mut query_builder = sqlx::query_as::<_, AgentRun>(&query);
        for role in roles {
            query_builder = query_builder.bind(role.to_string());
        }

        let runs = query_builder.fetch_all(pool).await?;
        Ok(runs)
    }

    pub async fn create(pool: &SqlitePool, run: &AgentRun) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO agent_runs (id, task_id, agent_role, status, runner_id, started_at, completed_at, patch_content, error_message, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&run.id)
        .bind(&run.task_id)
        .bind(&run.agent_role)
        .bind(&run.status)
        .bind(&run.runner_id)
        .bind(&run.started_at)
        .bind(&run.completed_at)
        .bind(&run.patch_content)
        .bind(&run.error_message)
        .bind(&run.created_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn claim_run(
        pool: &SqlitePool,
        id: &str,
        runner_id: &str,
    ) -> AppResult<AgentRun> {
        let started_at = Utc::now().to_rfc3339();

        let result = sqlx::query(
            r#"
            UPDATE agent_runs
            SET status = 'Running', runner_id = ?, started_at = ?
            WHERE id = ? AND status = 'Pending'
            "#,
        )
        .bind(runner_id)
        .bind(&started_at)
        .bind(id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Conflict(format!(
                "AgentRun {} is no longer available",
                id
            )));
        }

        Self::find_by_id(pool, id).await
    }

    pub async fn complete_run(
        pool: &SqlitePool,
        id: &str,
        status: AgentRunStatus,
        patch_content: Option<&str>,
        error_message: Option<&str>,
    ) -> AppResult<AgentRun> {
        let completed_at = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE agent_runs
            SET status = ?, completed_at = ?, patch_content = ?, error_message = ?
            WHERE id = ?
            "#,
        )
        .bind(status.to_string())
        .bind(&completed_at)
        .bind(patch_content)
        .bind(error_message)
        .bind(id)
        .execute(pool)
        .await?;

        Self::find_by_id(pool, id).await
    }

    pub async fn update_heartbeat(pool: &SqlitePool, runner_id: &str) -> AppResult<u64> {
        // This could be used to track active runners
        // For now, just return count of running tasks for this runner
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM agent_runs WHERE runner_id = ? AND status = 'Running'",
        )
        .bind(runner_id)
        .fetch_one(pool)
        .await?;

        Ok(count.0 as u64)
    }
}

pub struct ReasoningLogRepository;

impl ReasoningLogRepository {
    pub async fn find_by_run(pool: &SqlitePool, agent_run_id: &str) -> AppResult<Vec<ReasoningLog>> {
        let logs = sqlx::query_as::<_, ReasoningLog>(
            "SELECT * FROM reasoning_logs WHERE agent_run_id = ? ORDER BY step_number ASC",
        )
        .bind(agent_run_id)
        .fetch_all(pool)
        .await?;

        Ok(logs)
    }

    pub async fn create(pool: &SqlitePool, log: &ReasoningLog) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO reasoning_logs (id, agent_run_id, step_number, thought, action, observation, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&log.id)
        .bind(&log.agent_run_id)
        .bind(log.step_number)
        .bind(&log.thought)
        .bind(&log.action)
        .bind(&log.observation)
        .bind(&log.created_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn create_many(pool: &SqlitePool, logs: &[ReasoningLog]) -> AppResult<()> {
        for log in logs {
            Self::create(pool, log).await?;
        }
        Ok(())
    }
}
