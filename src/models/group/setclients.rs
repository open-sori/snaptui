use serde::{Deserialize, Serialize};
use crate::models::server::getstatus::{GroupData};

/// Request structure for Group.SetClients
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetClientsRequest {
    /// JSON-RPC ID field
    pub id: String,
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Parameters for the request
    pub params: SetClientsParams,
}

/// Parameters for Group.SetClients request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetClientsParams {
    /// Group ID
    pub id: String,
    /// List of client IDs
    pub clients: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetClientsResponse {
    pub id: String,
    pub jsonrpc: String,
    pub result: SetClientsResult,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetClientsResult {
    pub server: SetClientsResultServer,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetClientsResultServer {
    pub groups: Vec<GroupData>,
}