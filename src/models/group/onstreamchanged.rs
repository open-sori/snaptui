use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupOnStreamChanged {
    pub jsonrpc: String,
    pub method: String,
    pub params: GroupOnStreamChangedParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupOnStreamChangedParams {
    pub id: String,
    pub stream_id: String,
}