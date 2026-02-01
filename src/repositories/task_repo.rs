use chrono::Utc;
use sqlx::SqlitePool;

use crate::error::{AppError, AppResult};
use crate::models::{Task, TaskStatus, UpdateTaskRequest};

pub struct TaskRepository;

impl TaskRepository {
    pub async fn find_all_by_project(pool: &SqlitePool, project_id: &str) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE project_id = ? ORDER BY priority DESC, created_at DESC",
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> AppResult<Task> {
        sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task not found: {}", id)))
    }

    pub async fn find_by_status(
        pool: &SqlitePool,
        project_id: &str,
        status: TaskStatus,
    ) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE project_id = ? AND status = ? ORDER BY priority DESC",
        )
        .bind(project_id)
        .bind(status.to_string())
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    pub async fn find_children(pool: &SqlitePool, parent_id: &str) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE parent_task_id = ? ORDER BY priority DESC",
        )
        .bind(parent_id)
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    pub async fn create(pool: &SqlitePool, task: &Task) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO tasks (id, project_id, title, description, acceptance_criteria, status, priority, parent_task_id, branch_name, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&task.id)
        .bind(&task.project_id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.acceptance_criteria)
        .bind(&task.status)
        .bind(task.priority)
        .bind(&task.parent_task_id)
        .bind(&task.branch_name)
        .bind(&task.created_at)
        .bind(&task.updated_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(pool: &SqlitePool, id: &str, req: &UpdateTaskRequest) -> AppResult<Task> {
        let existing = Self::find_by_id(pool, id).await?;

        let title = req.title.as_ref().unwrap_or(&existing.title);
        let description = req.description.as_ref().unwrap_or(&existing.description);
        let acceptance_criteria = req
            .acceptance_criteria
            .as_ref()
            .unwrap_or(&existing.acceptance_criteria);
        let status = req
            .status
            .map(|s| s.to_string())
            .unwrap_or(existing.status.clone());
        let priority = req.priority.unwrap_or(existing.priority);
        let branch_name = req.branch_name.as_ref().or(existing.branch_name.as_ref());
        let updated_at = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE tasks
            SET title = ?, description = ?, acceptance_criteria = ?, status = ?, priority = ?, branch_name = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(title)
        .bind(description)
        .bind(acceptance_criteria)
        .bind(&status)
        .bind(priority)
        .bind(branch_name)
        .bind(&updated_at)
        .bind(id)
        .execute(pool)
        .await?;

        Self::find_by_id(pool, id).await
    }

    pub async fn update_status(
        pool: &SqlitePool,
        id: &str,
        status: TaskStatus,
    ) -> AppResult<Task> {
        let updated_at = Utc::now().to_rfc3339();

        sqlx::query("UPDATE tasks SET status = ?, updated_at = ? WHERE id = ?")
            .bind(status.to_string())
            .bind(&updated_at)
            .bind(id)
            .execute(pool)
            .await?;

        Self::find_by_id(pool, id).await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task not found: {}", id)));
        }

        Ok(())
    }
}
