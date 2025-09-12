use anyhow::Result;
use sqlx::{Row, SqlitePool};
use tracing::{info};

use crate::models::{Session, SessionIdleRequest};

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
