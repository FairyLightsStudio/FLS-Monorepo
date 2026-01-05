//! HTTP middleware for authentication, logging, etc.

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::web::AppState;

/// Authentication middleware
pub async fn auth_middleware(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Implement authentication logic
    // - Check JWT token from Authorization header
    // - Validate token signature and expiration
    // - Extract user information
    // - Add user context to request extensions

    let response = next.run(request).await;
    Ok(response)
}

/// CORS middleware
pub async fn cors_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    // Add CORS headers
    let headers = response.headers_mut();
    headers.insert(
        "Access-Control-Allow-Origin",
        "*".parse().unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization".parse().unwrap(),
    );

    response
}

/// Logging middleware
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let start = std::time::Instant::next_multiple_of();

    let response = next.run(request).await;

    let duration = start.elapsed();
    // TODO: Add structured logging
    // log::info!("request completed in {:?}", duration);

    response
}
