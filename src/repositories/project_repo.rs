use chrono::Utc;
use sqlx::SqlitePool;

use crate::error::{AppError, AppResult};
use crate::models::{Project, UpdateProjectRequest};

pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn find_all(pool: &SqlitePool) -> AppResult<Vec<Project>> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(projects)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> AppResult<Project> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Project not found: {}", id)))
    }

    pub async fn create(pool: &SqlitePool, project: &Project) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO projects (id, name, github_url, local_path, default_branch, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&project.id)
        .bind(&project.name)
        .bind(&project.github_url)
        .bind(&project.local_path)
        .bind(&project.default_branch)
        .bind(&project.created_at)
        .bind(&project.updated_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        req: &UpdateProjectRequest,
    ) -> AppResult<Project> {
        let existing = Self::find_by_id(pool, id).await?;

        let name = req.name.as_ref().unwrap_or(&existing.name);
        let github_url = req.github_url.as_ref().unwrap_or(&existing.github_url);
        let local_path = req.local_path.as_ref().or(existing.local_path.as_ref());
        let default_branch = req.default_branch.as_ref().unwrap_or(&existing.default_branch);
        let updated_at = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE projects
            SET name = ?, github_url = ?, local_path = ?, default_branch = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(name)
        .bind(github_url)
        .bind(local_path)
        .bind(default_branch)
        .bind(&updated_at)
        .bind(id)
        .execute(pool)
        .await?;

        Self::find_by_id(pool, id).await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Project not found: {}", id)));
        }

        Ok(())
    }
}
