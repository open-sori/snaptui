use serde::{Deserialize, Serialize};

/// Request structure for Group.SetMute
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetMuteRequest {
    /// JSON-RPC ID field
    pub id: String,
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Parameters for the request
    pub params: SetMuteParams,
}

/// Parameters for Group.SetMute request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetMuteParams {
    /// Group ID
    pub id: String,
    /// Mute status
    pub mute: bool,
}

/// Response structure for Group.SetMute
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetMuteResponse {
    /// JSON-RPC ID field
    pub id: String,
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Result containing the new mute status
    pub result: SetMuteResult,
}

/// Result containing the new mute status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetMuteResult {
    /// The new mute status that was set
    pub mute: bool,
}