use axum::{
    middleware,
    routing::{post, Router},
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::{
    handlers::{handle_embed, handle_message},
    middleware::{auth_middleware, logging_middleware},
    state::AppState,
};

/// API route paths
pub mod paths {
    pub const EMBED: &str = "/api/embed";
    pub const CHAT: &str = "/api/chat";
}

/// Creates the application router with all routes and middleware
pub fn create_router(state: Arc<AppState>) -> Router {
    // Create base router with routes
    let router = Router::new()
        .route(paths::EMBED, post(handle_embed))
        .route(paths::CHAT, post(handle_message));

    // Add middleware layers
    router
        // Global middleware
        .layer(TraceLayer::new_for_http())
        // Authentication middleware
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        // Logging middleware
        .route_layer(middleware::from_fn(logging_middleware))
        // Application state
        .with_state(state)
} 