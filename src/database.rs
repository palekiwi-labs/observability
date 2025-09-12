use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use tracing::{error, info};

use crate::models::{Session, SessionIdleRequest, SessionStatus};

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                title TEXT,
                hostname TEXT,
                created_at TEXT NOT NULL,
                completed_at TEXT,
                response TEXT,
                summary TEXT,
                status TEXT NOT NULL DEFAULT 'active'
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        info!("Database migrations completed");
        Ok(())
    }

    pub async fn get_sessions(&self, limit: Option<i64>) -> Result<Vec<Session>> {
        let limit = limit.unwrap_or(100);

        let rows = sqlx::query(
            "SELECT id, title, hostname, created_at, completed_at, response, summary, status
             FROM sessions
             ORDER BY created_at DESC
             LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut sessions = Vec::new();
        for row in rows {
            let status_str: String = row.get("status");
            let status = match status_str.as_str() {
                "active" => SessionStatus::Active,
                "completed" => SessionStatus::Completed,
                "failed" => SessionStatus::Failed,
                _ => SessionStatus::Active,
            };

            let created_at: String = row.get("created_at");
            let completed_at: Option<String> = row.get("completed_at");

            sessions.push(Session {
                id: row.get("id"),
                title: row.get("title"),
                hostname: row.get("hostname"),
                created_at: created_at.parse()?,
                completed_at: completed_at.map(|s| s.parse()).transpose()?,
                response: row.get("response"),
                summary: row.get("summary"),
                status,
            });
        }

        Ok(sessions)
    }

    pub async fn get_session(&self, id: &str) -> Result<Option<Session>> {
        let row = sqlx::query(
            "SELECT id, title, hostname, created_at, completed_at, response, summary, status
             FROM sessions
             WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let status_str: String = row.get("status");
            let status = match status_str.as_str() {
                "active" => SessionStatus::Active,
                "completed" => SessionStatus::Completed,
                "failed" => SessionStatus::Failed,
                _ => SessionStatus::Active,
            };

            let created_at: String = row.get("created_at");
            let completed_at: Option<String> = row.get("completed_at");

            Ok(Some(Session {
                id: row.get("id"),
                title: row.get("title"),
                hostname: row.get("hostname"),
                created_at: created_at.parse()?,
                completed_at: completed_at.map(|s| s.parse()).transpose()?,
                response: row.get("response"),
                summary: row.get("summary"),
                status,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_or_update_session(&self, session: &Session) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO sessions
            (id, title, hostname, created_at, completed_at, response, summary, status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&session.id)
        .bind(&session.title)
        .bind(&session.hostname)
        .bind(session.created_at.to_rfc3339())
        .bind(session.completed_at.map(|dt| dt.to_rfc3339()))
        .bind(&session.response)
        .bind(&session.summary)
        .bind(match session.status {
            SessionStatus::Active => "active",
            SessionStatus::Completed => "completed",
            SessionStatus::Failed => "failed",
        })
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn complete_session(
        &self,
        id: &str,
        request: SessionIdleRequest,
    ) -> Result<Option<Session>> {
        // First check if session exists or create it
        let mut session = if let Some(existing) = self.get_session(id).await? {
            existing
        } else {
            Session::new(id.to_string())
        };

        // Update with completion data
        session.complete(request);

        // Save to database
        self.create_or_update_session(&session).await?;

        Ok(Some(session))
    }
}
