use anyhow::Result;
use qdrant_client::{
    Qdrant,
    config::QdrantConfig,
    qdrant::{PointStruct, Vectors, Value as QdrantValue, WriteOrdering, DeletePoints, Filter, PointId, SearchPoints, SearchResponse, PointsSelector, points_selector::PointsSelectorOneOf},
};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::error::Error;

use crate::models::Document;
use crate::config::Config;

/// Service for interacting with the Qdrant vector database.
/// 
/// Provides functionality for storing and retrieving documents with their
/// associated embedding vectors. Handles connection management and CRUD
/// operations for vector search capabilities.
pub struct QdrantService {
    /// Client for communicating with the Qdrant server
    client: Qdrant,
    /// Name of the collection where documents are stored
    collection_name: String,
}

impl QdrantService {
    /// Creates a new QdrantService instance with the specified configuration.
    /// 
    /// # Arguments
    /// * `url` - Base URL of the Qdrant server (e.g., "http://localhost:6333")
    /// * `api_key` - Optional API key for authentication with Qdrant Cloud
    /// * `collection_name` - Name of the collection to use for document storage
    /// 
    /// # Returns
    /// * `Ok(Self)` - A configured QdrantService instance
    /// * `Err(anyhow::Error)` - If connection fails or configuration is invalid
    /// 
    /// # Example
    /// ```no_run
    /// let service = QdrantService::new(
    ///     "http://localhost:6333",
    ///     None, // No API key for local instance
    ///     "my_collection"
    /// )?;
    /// ```
    pub fn new(url: &str, api_key: Option<&str>, collection_name: &str) -> Result<Self> {
        // Initialize client configuration
        let mut config = QdrantConfig::from_url(url);
        
        // Configure API key if provided (required for Qdrant Cloud)
        if let Some(key) = api_key {
            config = config.api_key(key);
        }

        // Create client with configuration
        let client = Qdrant::new(config)?;

        // Return configured service instance
        Ok(Self {
            client,
            collection_name: collection_name.to_string(),
        })
    }

    /// Converts a JSON value to a Qdrant value.
    fn json_to_qdrant_value(value: &JsonValue) -> QdrantValue {
        match value {
            JsonValue::Null => QdrantValue {
                kind: Some(qdrant_client::qdrant::value::Kind::NullValue(0)),
            },
            JsonValue::Bool(b) => QdrantValue {
                kind: Some(qdrant_client::qdrant::value::Kind::BoolValue(*b)),
            },
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    QdrantValue {
                        kind: Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)),
                    }
                } else if let Some(f) = n.as_f64() {
                    QdrantValue {
                        kind: Some(qdrant_client::qdrant::value::Kind::DoubleValue(f)),
                    }
                } else {
                    QdrantValue {
                        kind: Some(qdrant_client::qdrant::value::Kind::NullValue(0)),
                    }
                }
            },
            JsonValue::String(s) => QdrantValue {
                kind: Some(qdrant_client::qdrant::value::Kind::StringValue(s.clone())),
            },
            JsonValue::Array(arr) => QdrantValue {
                kind: Some(qdrant_client::qdrant::value::Kind::ListValue(
                    qdrant_client::qdrant::ListValue {
                        values: arr.iter().map(|v| Self::json_to_qdrant_value(v)).collect(),
                    },
                )),
            },
            JsonValue::Object(obj) => {
                let mut struct_value = HashMap::new();
                for (k, v) in obj {
                    struct_value.insert(k.clone(), Self::json_to_qdrant_value(v));
                }
                QdrantValue {
                    kind: Some(qdrant_client::qdrant::value::Kind::StructValue(
                        qdrant_client::qdrant::Struct { fields: struct_value },
                    )),
                }
            },
        }
    }

    /// Stores or updates a document in the Qdrant collection.
    /// 
    /// This method performs an upsert operation, which means:
    /// - If a document with the same ID exists, it will be updated
    /// - If no document with the ID exists, a new one will be created
    /// 
    /// The document's embedding vector and metadata are stored together,
    /// allowing for vector similarity search with metadata filtering.
    /// 
    /// # Arguments
    /// * `doc` - Document containing the ID, embedding vector, and metadata
    /// 
    /// # Returns
    /// * `Ok(())` - Document was successfully stored
    /// * `Err(anyhow::Error)` - If the storage operation fails
    /// 
    /// # Example
    /// ```no_run
    /// let doc = Document {
    ///     id: "doc1".to_string(),
    ///     embedding: vec![0.1, 0.2, 0.3],
    ///     // ... other fields
    /// };
    /// service.upsert_document(&doc).await?;
    /// ```
    pub async fn upsert_document(&self, doc: &Document) -> Result<()> {
        use qdrant_client::qdrant::UpsertPoints;

        // Convert document to JSON value
        let json_value = serde_json::to_value(doc)?;
        
        // Convert JSON object to Qdrant payload
        let payload = match json_value {
            JsonValue::Object(obj) => obj.into_iter()
                .filter(|(k, _)| k != "embedding") // Skip embedding field
                .map(|(k, v)| (k, Self::json_to_qdrant_value(&v)))
                .collect(),
            _ => return Err(anyhow::anyhow!("Document serialization failed")),
        };

        // Construct the point structure for Qdrant
        let point = PointStruct {
            id: Some(doc.id.clone().into()),
            vectors: Some(Vectors::from(doc.embedding.clone())),
            payload,
        };

        // Create the upsert points operation
        let upsert_operation = UpsertPoints {
            collection_name: self.collection_name.clone(),
            points: vec![point],
            ordering: Some(WriteOrdering::default().into()),
            ..Default::default()
        };

        // Perform the upsert operation
        self.client
            .upsert_points(upsert_operation)
            .await?;

        Ok(())
    }

    /// Deletes all points from the collection.
    /// 
    /// This method effectively resets the collection by removing all stored vectors.
    /// 
    /// # Returns
    /// * `Ok(())` - If all points were deleted successfully
    /// * `Err(Box<dyn Error>)` - If the deletion fails
    pub async fn delete_all_points(&self) -> Result<(), Box<dyn Error>> {
        let points_selector = PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Filter(Filter::default())),
        };
        let delete_points = DeletePoints {
            collection_name: self.collection_name.clone(),
            points: Some(points_selector),
            ordering: Some(WriteOrdering::default().into()),
            ..Default::default()
        };
        self.client
            .delete_points(delete_points)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }
} 