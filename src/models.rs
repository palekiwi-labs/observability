use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub title: Option<String>,
    pub hostname: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub response: Option<String>,
    pub summary: Option<String>,
}



#[derive(Debug, Deserialize)]
pub struct SessionIdleRequest {
    pub title: Option<String>,
    pub hostname: Option<String>,
    pub response: Option<String>,
    pub summary: Option<String>,
}

impl Session {
    pub fn new(id: String) -> Self {
        Self {
            id,
            title: None,
            hostname: None,
            created_at: Utc::now(),
            completed_at: None,
            response: None,
            summary: None,
        }
    }

    pub fn complete(&mut self, request: SessionIdleRequest) {
        self.title = request.title;
        self.hostname = request.hostname;
        self.response = request.response;
        self.summary = request.summary;
        self.completed_at = Some(Utc::now());
    }
}
