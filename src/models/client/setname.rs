use serde::{Deserialize, Serialize};

/// Request structure for Client.SetName
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetNameRequest {
    /// JSON-RPC ID field
    pub id: String,
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Parameters for the request
    pub params: SetNameParams,
}

/// Parameters for Client.SetName request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetNameParams {
    /// Client ID to rename
    pub id: String,
    /// New name for the client
    pub name: String,
}

/// Response structure for Client.SetName
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetNameResponse {
    /// JSON-RPC ID field
    pub id: String,
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Result containing the new name
    pub result: SetNameResult,
}

/// Result containing the new name
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetNameResult {
    /// The new name that was set
    pub name: String,
}