use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnLatencyChanged {
    pub jsonrpc: String,
    pub method: String,
    pub params: ClientOnLatencyChangedParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnLatencyChangedParams {
    pub id: String,
    pub latency: i32,
}