use serde::{Deserialize, Serialize};
use crate::models::server::getstatus::GroupClient;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnDisconnect {
    pub jsonrpc: String,
    pub method: String,
    pub params: ClientOnDisconnectParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnDisconnectParams {
    pub id: String,
    pub client: GroupClient,
}