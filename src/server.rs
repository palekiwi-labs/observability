use axum::{
    Router,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{database::Database, handlers};

pub fn create_app(database: Database) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/sessions", get(handlers::get_sessions))
        .route("/sessions/:id/idle", post(handlers::session_idle))
        .route("/inspect", post(handlers::inspect_json))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(database)
}

