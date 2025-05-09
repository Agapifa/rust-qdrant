use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::state::AppState;

/// Middleware that validates the API key in the request header.
/// 
/// This middleware checks for the presence of an 'x-api-key' header and validates
/// its value against the configured API key. If the key is missing or invalid,
/// the request is rejected with a 401 Unauthorized status.
/// 
/// # Arguments
/// * `state` - Application state containing the valid API key
/// * `request` - The incoming HTTP request
/// * `next` - The next middleware in the chain
/// 
/// # Returns
/// * `Ok(Response)` - If authentication succeeds
/// * `Err(StatusCode)` - If authentication fails
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract and validate the API key from the request header
    let api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            warn!("Missing API key in request to {}", request.uri());
            StatusCode::UNAUTHORIZED
        })?;

    // Check if the provided API key matches the configured one
    if api_key != state.config.api_key {
        warn!("Invalid API key provided for {}", request.uri());
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Log successful authentication with request details
    info!(
        method = %request.method(),
        uri = %request.uri(),
        "Request authenticated successfully"
    );
    
    // Continue processing the request
    Ok(next.run(request).await)
}

/// Middleware that logs request and response details.
/// 
/// This middleware captures timing information and logs details about incoming
/// requests and their corresponding responses. It includes HTTP method, URI,
/// status code, and request duration.
/// 
/// # Arguments
/// * `request` - The incoming HTTP request
/// * `next` - The next middleware in the chain
/// 
/// # Returns
/// * `Ok(Response)` - The processed response
/// * `Err(StatusCode)` - If an error occurs during processing
pub async fn logging_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Store request details and start timing
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();

    // Log incoming request details
    info!(
        method = %method,
        uri = %uri,
        "Incoming request"
    );

    // Process the request and measure duration
    let response = next.run(request).await;
    let duration = start.elapsed();

    // Log response details with appropriate level based on status
    if response.status().is_success() {
        info!(
            method = %method,
            uri = %uri,
            status = %response.status(),
            duration = ?duration,
            "Request completed successfully"
        );
    } else {
        error!(
            method = %method,
            uri = %uri,
            status = %response.status(),
            duration = ?duration,
            "Request failed"
        );
    }

    Ok(response)
} 