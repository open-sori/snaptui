use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamOnProperties {
    pub jsonrpc: String,
    pub method: String,
    pub params: StreamOnPropertiesParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamOnPropertiesParams {
    pub id: String,
    pub metadata: Value,
}