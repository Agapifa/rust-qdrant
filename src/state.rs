use crate::{config::Config, services::{OpenAIService, QdrantService}};

/// Application state shared across all requests.
/// 
/// This struct holds instances of all services and configuration
/// needed by the application. It is wrapped in an Arc and shared
/// across all request handlers.
pub struct AppState {
    /// Application configuration
    pub config: Config,
    /// OpenAI service for embeddings and chat
    pub openai_service: OpenAIService,
    /// Qdrant service for vector storage
    pub qdrant_service: QdrantService,
}

impl AppState {
    /// Creates a new instance of AppState.
    /// 
    /// This constructor takes ownership of all required services
    /// and configuration, creating a new application state instance.
    /// 
    /// # Arguments
    /// * `config` - Application configuration
    /// * `openai_service` - Initialized OpenAI service
    /// * `qdrant_service` - Initialized Qdrant service
    /// 
    /// # Returns
    /// A new AppState instance
    pub fn new(
        config: Config,
        openai_service: OpenAIService,
        qdrant_service: QdrantService,
    ) -> Self {
        Self {
            config,
            openai_service,
            qdrant_service,
        }
    }
} 