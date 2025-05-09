/// Configuration module for environment variables and settings
mod config;
/// Request handlers for API endpoints
mod handlers;
/// Middleware for authentication and logging
mod middleware;
/// Database models and schemas
mod models;
/// API route definitions
mod routes;
/// External service integrations
mod services;
/// Application state management
mod state;
/// Shared types and API contracts
mod types;

use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::Config,
    services::{OpenAIService, QdrantService},
    state::AppState,
};

/// Application entry point.
/// 
/// This function performs the following setup:
/// 1. Initializes logging with tracing
/// 2. Loads environment variables
/// 3. Creates service instances
/// 4. Sets up the web server
/// 
/// # Returns
/// * `Result<()>` - Ok if server starts successfully, Err otherwise
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for structured logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Load application configuration
    let config = Config::from_env()?;
    
    // Initialize external services
    let openai_service = OpenAIService::new(&config.openai_api_key);
    let qdrant_service = QdrantService::new(
        &config.qdrant_url,
        config.qdrant_api_key.as_deref(),
        &config.collection_name,
    )?;

    // Create shared application state
    let state = Arc::new(AppState::new(config, openai_service, qdrant_service));
    
    // Create router with all routes and middleware
    let app = routes::create_router(state);
    
    // Configure and start the server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    // Start serving requests
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
