use axum::{extract::State, http::StatusCode, Json};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};

use crate::{
    state::AppState,
    types::{ApiResponse, EmbeddingRequest, MessageRequest},
};

/// Handles requests to generate embeddings from text input.
/// 
/// # Arguments
/// * `state` - Application state containing service instances
/// * `payload` - JSON payload containing the text to embed
/// 
/// # Returns
/// * `Ok(Json<ApiResponse<Vec<f32>>>)` - Vector of floating point numbers representing the embedding
/// * `Err(StatusCode)` - Error status code if the request fails
/// 
/// # Example Request
/// ```json
/// {
///     "text": "Your text to embed"
/// }
/// ```
pub async fn handle_embed(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EmbeddingRequest>,
) -> Result<Json<ApiResponse<Vec<f32>>>, StatusCode> {
    // Validate that the input text is not empty
    if payload.text.trim().is_empty() {
        error!("Empty text provided for embedding");
        return Ok(Json(ApiResponse::<Vec<f32>>::error("Text cannot be empty".into())));
    }

    // Call OpenAI service to generate embedding
    let embedding = state
        .openai_service
        .get_embedding(&payload.text)
        .await
        .map_err(|e| {
            error!("Failed to generate embedding: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Log success and return the embedding
    info!("Successfully generated embedding for text length: {}", payload.text.len());
    Ok(Json(ApiResponse::success(embedding)))
}

/// Handles chat message requests to generate AI responses.
/// 
/// # Arguments
/// * `state` - Application state containing service instances
/// * `payload` - JSON payload containing the message to process
/// 
/// # Returns
/// * `Ok(Json<ApiResponse<Value>>)` - JSON response containing the AI-generated message
/// * `Err(StatusCode)` - Error status code if the request fails
/// 
/// # Example Request
/// ```json
/// {
///     "message": "What is the capital of France?"
/// }
/// ```
pub async fn handle_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MessageRequest>,
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    // Validate that the input message is not empty
    if payload.message.trim().is_empty() {
        error!("Empty message provided");
        return Ok(Json(ApiResponse::<Value>::error("Message cannot be empty".into())));
    }

    // Call OpenAI service to generate completion
    let response = state
        .openai_service
        .generate_completion(&payload.message)
        .await
        .map_err(|e| {
            error!("Failed to generate completion: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Log success with token usage
    info!(
        "Successfully generated completion with {} tokens",
        response.usage.total_tokens
    );

    // Return the formatted response
    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": response.response,
        "usage": response.usage
    }))))
}

/// Handles database reset requests.
/// 
/// This endpoint clears all data from the Qdrant collection,
/// effectively resetting the database to its initial state.
/// 
/// # Arguments
/// * `state` - Application state containing service instances
/// 
/// # Returns
/// * `Ok(Json<ApiResponse<Value>>)` - Success message
/// * `Err(StatusCode)` - Error status code if the reset fails
pub async fn handle_reset(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    // Delete all points from the collection
    state
        .qdrant_service
        .delete_all_points()
        .await
        .map_err(|e| {
            error!("Failed to reset database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Log success
    info!("Database reset successfully");

    // Return success message
    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Database reset successfully"
    }))))
} 