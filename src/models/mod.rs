use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: u64,
    pub text: String,
    pub embedding: Vec<f32>,
} 