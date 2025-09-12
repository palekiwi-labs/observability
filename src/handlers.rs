use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{database::Database, models::SessionIdleRequest};

#[derive(Deserialize)]
pub struct SessionsQuery {
    limit: Option<i64>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    timestamp: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

pub async fn get_sessions(
    State(db): State<Database>,
    Query(params): Query<SessionsQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match db.get_sessions(params.limit).await {
        Ok(sessions) => {
            info!("Retrieved {} sessions", sessions.len());
            Ok(Json(serde_json::json!({
                "sessions": sessions,
                "count": sessions.len()
            })))
        }
        Err(e) => {
            error!("Failed to retrieve sessions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn session_idle(
    State(db): State<Database>,
    Path(session_id): Path<String>,
    Json(request): Json<SessionIdleRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match db.complete_session(&session_id, request).await {
        Ok(Some(session)) => {
            info!("Session {} marked as completed", session_id);
            Ok(Json(serde_json::json!({
                "status": "success",
                "session": session
            })))
        }
        Ok(None) => {
            error!("Failed to create or update session {}", session_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(e) => {
            error!("Failed to complete session {}: {}", session_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
