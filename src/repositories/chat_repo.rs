use sqlx::SqlitePool;

use crate::error::AppResult;
use crate::models::ChatMessage;

pub struct ChatRepository;

impl ChatRepository {
    pub async fn find_by_project(
        pool: &SqlitePool,
        project_id: &str,
        limit: Option<i64>,
    ) -> AppResult<Vec<ChatMessage>> {
        let limit = limit.unwrap_or(100);
        let messages = sqlx::query_as::<_, ChatMessage>(
            "SELECT * FROM chat_messages WHERE project_id = ? ORDER BY created_at DESC LIMIT ?",
        )
        .bind(project_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        // Return in chronological order
        let mut messages = messages;
        messages.reverse();
        Ok(messages)
    }

    pub async fn find_by_task(
        pool: &SqlitePool,
        task_id: &str,
        limit: Option<i64>,
    ) -> AppResult<Vec<ChatMessage>> {
        let limit = limit.unwrap_or(100);
        let messages = sqlx::query_as::<_, ChatMessage>(
            "SELECT * FROM chat_messages WHERE task_id = ? ORDER BY created_at DESC LIMIT ?",
        )
        .bind(task_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let mut messages = messages;
        messages.reverse();
        Ok(messages)
    }

    pub async fn create(pool: &SqlitePool, message: &ChatMessage) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO chat_messages (id, task_id, project_id, sender_type, sender_name, content, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&message.id)
        .bind(&message.task_id)
        .bind(&message.project_id)
        .bind(&message.sender_type)
        .bind(&message.sender_name)
        .bind(&message.content)
        .bind(&message.created_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn create_many(pool: &SqlitePool, messages: &[ChatMessage]) -> AppResult<()> {
        for message in messages {
            Self::create(pool, message).await?;
        }
        Ok(())
    }
}
