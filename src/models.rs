use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct InputPayload {
    pub word: String,
    pub colors: String,
}

#[derive(Debug, Serialize)]
pub struct OutputPayload {
    pub words: Vec<String>,
}