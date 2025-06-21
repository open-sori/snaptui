use serde::{Deserialize, Serialize};

/// Request structure for Group.SetStream
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetStreamRequest {
    /// JSON-RPC ID field
    pub id: String,
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Parameters for the request
    pub params: SetStreamParams,
}

/// Parameters for Group.SetStream request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetStreamParams {
    /// Group ID
    pub id: String,
    /// New stream ID
    pub stream_id: String,
}

/// Response structure for Group.SetStream
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetStreamResponse {
    /// JSON-RPC ID field
    pub id: String,
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Result containing the new stream_id
    pub result: SetStreamResult,
}

/// Result containing the new stream_id
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetStreamResult {
    /// The new stream_id that was set
    pub stream_id: String,
}