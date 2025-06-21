use serde::{Deserialize, Serialize};

/// Request structure for Client.SetLatency
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetLatencyRequest {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: SetLatencyParams,
}

/// Parameters for Client.SetLatency request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetLatencyParams {
    pub id: String,
    pub latency: i32,
}

/// Response structure for Client.SetLatency
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetLatencyResponse {
    pub id: String,
    pub jsonrpc: String,
    pub result: SetLatencyResult,
}

/// Result containing the new latency
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetLatencyResult {
    pub latency: i32,
}