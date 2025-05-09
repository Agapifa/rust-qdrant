use anyhow::Result;
use std::env;

pub struct Config {
    pub openai_api_key: String,
    pub qdrant_url: String,
    pub qdrant_api_key: Option<String>,
    pub collection_name: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            openai_api_key: env::var("OPENAI_API_KEY")?,
            qdrant_url: env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string()),
            qdrant_api_key: env::var("QDRANT_API_KEY").ok(),
            collection_name: env::var("COLLECTION_NAME").unwrap_or_else(|_| "documents".to_string()),
            api_key: env::var("API_KEY")?,
        })
    }
} 