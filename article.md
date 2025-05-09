# Building a Vector Search API with Rust and Qdrant

## Introduction

In this article, we'll explore how to build a powerful vector search API using Rust and Qdrant. This project demonstrates how to create a modern, efficient, and secure API that combines the speed of Rust with the vector search capabilities of Qdrant, enhanced by OpenAI's embedding and chat completion features.

## Project Overview

The project implements a REST API with two main endpoints:
- `/api/embed`: Generates vector embeddings for text using OpenAI's API
- `/api/chat`: Provides chat completions using OpenAI's GPT models

## Key Features

- **Fast and Efficient**: Built with Rust for maximum performance
- **Vector Search**: Powered by Qdrant for efficient similarity search
- **AI Integration**: Leverages OpenAI's API for embeddings and chat completions
- **Secure**: Implements API key authentication
- **Production-Ready**: Includes logging, error handling, and proper API documentation

## Technical Stack

- **Backend**: Rust with Axum web framework
- **Vector Database**: Qdrant
- **AI Services**: OpenAI API
- **Authentication**: Custom API key middleware
- **Logging**: Structured logging with tracing

## Project Structure

```
src/
├── api/
│   └── routes.rs
├── handlers/
│   ├── chat.rs
│   └── embed.rs
├── middleware/
│   └── auth.rs
├── services/
│   ├── openai.rs
│   └── qdrant.rs
├── state.rs
├── types/
│   └── mod.rs
└── main.rs
```

## Core Components

### 1. API Routes

The API is organized into two main endpoints:
- Embedding endpoint for vector generation
- Chat endpoint for AI-powered conversations

### 2. Services

#### OpenAI Service
- Handles communication with OpenAI's API
- Manages embeddings and chat completions
- Implements proper error handling and retries

#### Qdrant Service
- Manages vector storage and retrieval
- Handles document upserting and searching
- Implements efficient vector operations

### 3. Authentication

The API implements a secure authentication system using API keys:
- Middleware-based authentication
- Secure key validation
- Proper error handling for unauthorized requests

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Qdrant server
- OpenAI API key

### Environment Setup

Create a `.env` file with the following variables:
```env
OPENAI_API_KEY=your_openai_api_key
QDRANT_URL=http://localhost:6333
API_KEY=your_api_key_here
```

### Running the Project

1. Start the Qdrant server:
```bash
docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant
```

2. Run the application:
```bash
cargo run
```

### Step-by-Step Code Setup

1. **Initialize the Project**
```bash
cargo new rust-qdrant
cd rust-qdrant
```

2. **Add Dependencies**
Add the following to your `Cargo.toml`:
```toml
[package]
name = "rust-qdrant"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower-http = { version = "0.5", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
dotenv = "0.15"
async-trait = "0.1"
reqwest = { version = "0.11", features = ["json"] }
qdrant-client = "1.7"
```

3. **Create Project Structure**
```bash
mkdir -p src/{api,handlers,middleware,services,types}
touch src/api/routes.rs
touch src/handlers/{chat.rs,embed.rs}
touch src/middleware/auth.rs
touch src/services/{openai.rs,qdrant.rs}
touch src/types/mod.rs
touch src/state.rs
```

4. **Implement Core Components**

a. **State Management** (`src/state.rs`):
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use qdrant_client::client::QdrantClient;

pub struct AppState {
    pub qdrant: Arc<RwLock<QdrantClient>>,
    pub openai_api_key: String,
}

impl AppState {
    pub fn new(qdrant: QdrantClient, openai_api_key: String) -> Self {
        Self {
            qdrant: Arc::new(RwLock::new(qdrant)),
            openai_api_key,
        }
    }
}
```

b. **Types** (`src/types/mod.rs`):
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub error: Option<String>,
}
```

c. **Authentication Middleware** (`src/middleware/auth.rs`):
```rust
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn auth_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let api_key = req.headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let expected_key = env::var("API_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if api_key != expected_key {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
```

d. **OpenAI Service** (`src/services/openai.rs`):
```rust
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct OpenAIService {
    client: Client,
    api_key: String,
}

impl OpenAIService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn create_embedding(&self, text: &str) -> Result<Vec<f32>, anyhow::Error> {
        let response = self.client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": "text-embedding-ada-002",
                "input": text
            }))
            .send()
            .await?;

        let embedding = response.json::<serde_json::Value>().await?;
        Ok(embedding["data"][0]["embedding"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect())
    }
}
```

e. **Qdrant Service** (`src/services/qdrant.rs`):
```rust
use qdrant_client::client::QdrantClient;
use qdrant_client::qdrant::{PointStruct, SearchPoints};

pub struct QdrantService {
    client: QdrantClient,
}

impl QdrantService {
    pub fn new(client: QdrantClient) -> Self {
        Self { client }
    }

    pub async fn upsert_vector(
        &self,
        collection_name: &str,
        id: u64,
        vector: Vec<f32>,
    ) -> Result<(), anyhow::Error> {
        let point = PointStruct {
            id: Some(id.into()),
            vectors: Some(vector.into()),
            payload: None,
        };

        self.client
            .upsert_points(collection_name, vec![point], None)
            .await?;

        Ok(())
    }
}
```

f. **API Routes** (`src/api/routes.rs`):
```rust
use axum::{
    routing::{post, get},
    Router,
};
use crate::handlers::{chat, embed};
use crate::middleware::auth;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/embed", post(embed::handle_embed))
        .route("/api/chat", post(chat::handle_chat))
        .layer(axum::middleware::from_fn(auth::auth_middleware))
}
```

g. **Main Application** (`src/main.rs`):
```rust
mod api;
mod handlers;
mod middleware;
mod services;
mod state;
mod types;

use axum::Router;
use dotenv::dotenv;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv().ok();

    // Initialize Qdrant client
    let qdrant_url = env::var("QDRANT_URL").expect("QDRANT_URL must be set");
    let qdrant_client = qdrant_client::client::QdrantClient::new(Some(qdrant_url));

    // Initialize state
    let state = state::AppState::new(
        qdrant_client,
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
    );

    // Create router
    let app = api::routes::create_router()
        .with_state(state);

    // Start server
    let addr = "127.0.0.1:3000";
    tracing::info!("Starting server on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

5. **Run and Test**
After implementing all components, you can run the application:
```bash
cargo run
```

The API will be available at `http://127.0.0.1:3000` with the following endpoints:
- POST `/api/embed` - Generate embeddings
- POST `/api/chat` - Chat completions

## Best Practices Implemented

1. **Error Handling**
   - Custom error types
   - Proper error propagation
   - User-friendly error messages

2. **Logging**
   - Structured logging with tracing
   - Request/response logging
   - Performance metrics

3. **Security**
   - API key authentication
   - Secure environment variable handling
   - Input validation

4. **Code Organization**
   - Modular architecture
   - Clear separation of concerns
   - Reusable components

## Performance Considerations

- Efficient vector operations
- Proper connection pooling
- Optimized request handling
- Minimal memory footprint

## Future Improvements

1. **Features**
   - Add more vector search capabilities
   - Implement batch operations
   - Add caching layer

2. **Infrastructure**
   - Add Docker support
   - Implement CI/CD pipeline
   - Add monitoring and metrics

3. **Documentation**
   - Add OpenAPI/Swagger documentation
   - Create more detailed API documentation
   - Add performance benchmarks

## Conclusion

This project demonstrates how to build a modern, efficient, and secure vector search API using Rust and Qdrant. It combines the best of both worlds: Rust's performance and safety with Qdrant's powerful vector search capabilities, enhanced by OpenAI's AI features.

The modular architecture and clean code organization make it easy to extend and maintain, while the implemented best practices ensure reliability and security.

## Resources

- [Rust Documentation](https://www.rust-lang.org/learn)
- [Qdrant Documentation](https://qdrant.tech/documentation/)
- [OpenAI API Documentation](https://platform.openai.com/docs/api-reference)
- [Axum Documentation](https://docs.rs/axum/latest/axum/) 