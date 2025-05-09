use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, CreateChatCompletionRequest,
        CreateEmbeddingRequest, EmbeddingInput,
        ChatCompletionRequestUserMessageContent,
    },
    Client,
};
use serde::{Deserialize, Serialize};

/// Model configuration for OpenAI API calls.
/// These constants define the specific models and parameters used.
pub mod models {
    /// GPT-4 Turbo model for chat completions (latest version)
    pub const CHAT_MODEL: &str = "gpt-4";
    /// Text embedding model (latest version)
    pub const EMBEDDING_MODEL: &str = "text-embedding-3-large";         
    /// Temperature for response generation (0.0 = deterministic, 1.0 = creative)
    pub const TEMPERATURE: f32 = 0.7;
}

/// Response structure for chat completion requests.
/// 
/// Contains both the generated response text and usage statistics
/// for token consumption tracking.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// The generated response text from the model
    pub response: String,
    /// Token usage statistics for the request
    pub usage: Usage,
}

/// Token usage statistics for API requests.
/// 
/// Tracks the number of tokens used in both the prompt and response,
/// useful for monitoring API usage and costs.
#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    /// Number of tokens in the input prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the generated response
    pub completion_tokens: u32,
    /// Total tokens used in the request
    pub total_tokens: u32,
}

/// Service for interacting with OpenAI's API.
/// 
/// This service provides methods for:
/// - Generating text embeddings using text-embedding-3-large
/// - Creating chat completions using GPT-4 Turbo
/// 
/// It handles authentication and request configuration automatically.
pub struct OpenAIService {
    /// OpenAI API client instance
    client: Client<OpenAIConfig>,
}

impl OpenAIService {
    /// Creates a new OpenAIService instance.
    /// 
    /// # Arguments
    /// * `api_key` - OpenAI API key for authentication
    /// 
    /// # Returns
    /// A new OpenAIService instance configured with the provided API key
    pub fn new(api_key: &str) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        Self {
            client: Client::with_config(config),
        }
    }

    /// Generates an embedding vector for the given text.
    /// 
    /// Uses OpenAI's text-embedding-3-large model to create
    /// a high-quality vector representation of the input text.
    /// 
    /// # Arguments
    /// * `text` - The text to convert into an embedding
    /// 
    /// # Returns
    /// * `Ok(Vec<f32>)` - The embedding vector on success
    /// * `Err(anyhow::Error)` - If the API request fails
    /// 
    /// # Example
    /// ```no_run
    /// let embedding = service.get_embedding("Hello, world!").await?;
    /// ```
    pub async fn get_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Create the embedding request with model configuration
        let request = CreateEmbeddingRequest {
            model: models::EMBEDDING_MODEL.into(),
            input: EmbeddingInput::String(text.to_string()),
            encoding_format: None,
            dimensions: None,
            user: None,
        };

        // Send request to OpenAI API
        let response = self.client.embeddings().create(request).await?;
        
        // Return the first (and only) embedding
        Ok(response.data[0].embedding.clone())
    }

    /// Generates a chat completion response for the given message.
    /// 
    /// Uses GPT-4 Turbo to generate a response to the input message,
    /// with predefined settings for token limit and temperature.
    /// 
    /// # Arguments
    /// * `message` - The user's input message
    /// 
    /// # Returns
    /// * `Ok(CompletionResponse)` - The generated response and usage stats
    /// * `Err(anyhow::Error)` - If the API request fails
    /// 
    /// # Example
    /// ```no_run
    /// let response = service.generate_completion("What is Rust?").await?;
    /// println!("Response: {}", response.response);
    /// println!("Total tokens: {}", response.usage.total_tokens);
    /// ```
    pub async fn generate_completion(&self, message: &str) -> Result<CompletionResponse> {
        // Create the chat completion request with model and parameters
        let request = CreateChatCompletionRequest {
            model: models::CHAT_MODEL.into(),
            messages: vec![ChatCompletionRequestMessage::User(
                async_openai::types::ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(message.to_string()),
                    name: None,
                }
            )],
            temperature: Some(models::TEMPERATURE),
            ..Default::default()
        };

        // Send request to OpenAI API
        let response = self.client.chat().create(request).await?;
        
        // Format and return the response
        Ok(CompletionResponse {
            response: response.choices[0].message.content.clone().unwrap_or_default(),
            usage: Usage {
                prompt_tokens: response.usage.as_ref().map_or(0, |u| u.prompt_tokens),
                completion_tokens: response.usage.as_ref().map_or(0, |u| u.completion_tokens),
                total_tokens: response.usage.as_ref().map_or(0, |u| u.total_tokens),
            },
        })
    }
} 