use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request payload for chat message endpoints.
/// 
/// This struct represents the JSON payload for sending messages
/// to the chat completion endpoint.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MessageRequest {
    /// The message text to be processed.
    /// Must not be empty.
    #[validate(length(min = 1, message = "Message cannot be empty"))]
    pub message: String,
}

/// Request payload for embedding generation endpoints.
/// 
/// This struct represents the JSON payload for generating
/// text embeddings using OpenAI's API.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EmbeddingRequest {
    /// The text to be converted into an embedding vector.
    /// Must not be empty.
    #[validate(length(min = 1, message = "Text cannot be empty"))]
    pub text: String,
}

/// Generic API response wrapper.
/// 
/// This struct provides a consistent response format for all API endpoints,
/// including success/error status and optional error messages.
/// 
/// # Type Parameters
/// * `T` - The type of data being returned
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// The response payload
    pub data: T,
    /// Response status ("success" or "error")
    pub status: String,
    /// Optional error message, only present on error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T: Default> ApiResponse<T> {
    /// Creates a successful response with the provided data.
    /// 
    /// # Arguments
    /// * `data` - The data to include in the response
    /// 
    /// # Returns
    /// A new ApiResponse instance with success status
    pub fn success(data: T) -> Self {
        Self {
            data,
            status: "success".to_string(),
            error: None,
        }
    }

    /// Creates an error response with the provided message.
    /// 
    /// # Arguments
    /// * `error` - The error message
    /// 
    /// # Returns
    /// A new ApiResponse instance with error status
    pub fn error(error: String) -> Self {
        Self {
            data: T::default(),
            status: "error".to_string(),
            error: Some(error),
        }
    }
}

/// Enumeration of possible API errors.
/// 
/// This enum represents the different types of errors that can occur
/// during API request processing.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Authentication-related errors
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Request validation errors
    #[error("Invalid request: {0}")]
    Validation(String),

    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(String),
} 