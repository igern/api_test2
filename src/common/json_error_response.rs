use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonErrorResponse {
    pub code: String,
    pub message: Option<String>,
}
