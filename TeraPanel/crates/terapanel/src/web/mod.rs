//! Web panel API module
//!
//! This module provides the HTTP API for the TeraPanel web interface.

pub mod routes;
pub mod middleware;

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};

use crate::config::Config;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: Config,
}

/// Create and configure the web router
pub async fn create_router(config: Config) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(AppState { config })
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Result<Json<String>, StatusCode> {
    Ok(Json("OK".to_string()))
}
