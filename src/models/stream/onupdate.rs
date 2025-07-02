use serde::{Deserialize, Serialize};
use crate::models::server::getstatus::StreamData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamOnUpdate {
    pub jsonrpc: String,
    pub method: String,
    pub params: StreamOnUpdateParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamOnUpdateParams {
    pub id: String,
    pub stream: StreamData,
}