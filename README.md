# Rust Qdrant AI Agent

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://semver.org)
[![Rust](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A secure and efficient Rust-based API service that combines OpenAI's embedding capabilities with Qdrant's vector database for building AI-powered applications.

## Version Information

- **Current Version**: 0.1.0
- **Rust Edition**: 2021
- **Minimum Rust Version**: 1.75.0
- **Dependencies**:
  - qdrant-client: 1.7.0
  - async-openai: 0.17.0
  - axum: 0.7
  - tokio: 1.36

## Package Goals

### 1. Secure API Service
- 🔐 API key authentication for all endpoints
- 🔒 Secure handling of OpenAI and Qdrant credentials
- 🛡️ Middleware-based request validation
- 📝 Request/response logging and tracing

### 2. Vector Embedding Management
- 🤖 OpenAI integration for text embeddings
- 📊 Efficient vector storage with Qdrant
- 🔄 Real-time embedding generation
- 📈 Scalable vector search capabilities

### 3. Developer Experience
- 🚀 Easy setup and configuration
- 📚 Comprehensive documentation
- 🧪 Built-in error handling
- 🔍 Detailed logging and tracing

### 4. Performance
- ⚡ Async/await for non-blocking operations
- 🏃‍♂️ Efficient memory management
- 📦 Minimal resource footprint
- 🔄 Optimized request handling

## Prerequisites

- Rust (latest stable version)
- Qdrant server running locally or accessible via network
- OpenAI API key
- Qdrant API key (optional, if using cloud version)

## Setup

1. Install Rust if you haven't already:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Start Qdrant server (if running locally):
```bash
docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant
```

3. Create a `.env` file in the project root and add your configuration:
```bash
# Required
OPENAI_API_KEY=your-openai-api-key-here
API_KEY=your-api-key-for-client-authentication

# Optional (if using Qdrant Cloud)
QDRANT_API_KEY=your-qdrant-api-key-here

# Optional (defaults shown)
QDRANT_URL=http://localhost:6333
COLLECTION_NAME=documents
RUST_LOG=info
```

4. Build and run the project:
```bash
cargo run
```

## API Usage

The server runs on `http://localhost:3000` and requires API key authentication via the `x-api-key` header.

### Generate Embeddings

```bash
curl -X POST http://localhost:3000/api/embed \
  -H "Content-Type: application/json" \
  -H "x-api-key: your-api-key-here" \
  -d '{"text": "Your text to embed"}'
```

Response:
```json
{
  "data": {
    "embedding": [0.1, 0.2, ...]
  },
  "status": "success"
}
```

### Send Messages to GPT-4

Send messages to GPT-4 and receive AI-generated responses:

```bash
curl -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -H "x-api-key: your-api-key-here" \
  -d '{
    "message": "What is the capital of France?"
  }'
```

Response:
```json
{
  "data": {
    "message": "The capital of France is Paris...",
    "usage": {
      "prompt_tokens": 7,
      "completion_tokens": 5,
      "total_tokens": 12
    }
  },
  "status": "success"
}
```

The chat endpoint uses predefined settings:
- Model: GPT-4
- Max Tokens: 1000
- Temperature: 0.7

## Project Structure

```
src/
├── config/
│   └── mod.rs         # Environment configuration and settings
├── handlers/
│   └── mod.rs         # API endpoint handlers
├── middleware/
│   └── mod.rs         # Authentication and request processing
├── models/
│   └── mod.rs         # Database models and schemas
├── services/
│   ├── mod.rs         # Service layer exports
│   ├── openai.rs      # OpenAI integration
│   └── qdrant.rs      # Qdrant integration
├── types/
│   └── mod.rs         # Shared types and API contracts
├── routes.rs          # API route definitions
├── state.rs           # Application state management
└── main.rs            # Application entry point
```

### Module Descriptions

#### Core Modules
- **config**: Environment variable handling and application configuration
- **state**: Application state and service initialization
- **types**: Shared data structures and API contracts

#### API Layer
- **routes**: Route definitions and middleware configuration
- **handlers**: Request handling and business logic
- **middleware**: Authentication and request processing

#### Service Layer
- **services/openai**: OpenAI API integration for embeddings and chat
- **services/qdrant**: Vector database operations
- **models**: Data models and database schemas

## Features

- Secure API key authentication for client requests
- Secure API key authentication for OpenAI and Qdrant
- Generate embeddings using OpenAI's API
- Create and manage collections
- Store and retrieve vector embeddings
- Handle document metadata
- Request logging and tracing

## Package Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| qdrant-client | 1.7.0 | Vector database client for storage and search |
| async-openai | 0.17.0 | OpenAI API client for text embeddings |
| axum | 0.7 | Web framework for HTTP routing |
| tokio | 1.36 | Async runtime with full features |
| serde | 1.0 | Serialization framework with derive macros |
| serde_json | 1.0 | JSON parsing and serialization |
| anyhow | 1.0 | Flexible error handling |
| async-trait | 0.1 | Async trait support |
| dotenv | 0.15 | Environment variable management |
| tower | 0.4 | Middleware framework |
| tower-http | 0.5 | HTTP middleware with tracing |
| tracing | 0.1 | Structured logging framework |
| tracing-subscriber | 0.3 | Logging configuration |

### Dependency Categories

1. **Core Services**
   - qdrant-client: Vector database operations
   - async-openai: AI model integration
   - axum: Web server framework

2. **Async & Concurrency**
   - tokio: Async runtime
   - async-trait: Async trait support

3. **Data Processing**
   - serde: Data serialization
   - serde_json: JSON handling

4. **Error Handling & Config**
   - anyhow: Error management
   - dotenv: Environment config

5. **Logging & Middleware**
   - tower: Middleware components
   - tower-http: HTTP middleware
   - tracing: Application logging
   - tracing-subscriber: Log configuration

## Dependencies

- qdrant-client: Official Qdrant client for Rust
- async-openai: OpenAI API client for Rust
- axum: Web framework
- tower: Middleware framework
- tokio: Async runtime
- serde: Serialization/deserialization
- anyhow: Error handling
- async-trait: Async trait support
- dotenv: Environment variable management
- tracing: Logging and tracing

## Future Roadmap

- [ ] Add rate limiting
- [ ] Implement vector search endpoints
- [ ] Add batch processing capabilities
- [ ] Implement caching layer
- [ ] Add OpenAPI/Swagger documentation
- [ ] Add health check endpoints
- [ ] Implement metrics collection
- [ ] Add more authentication methods 