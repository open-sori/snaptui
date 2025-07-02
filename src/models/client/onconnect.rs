use serde::{Deserialize, Serialize};
use crate::models::server::getstatus::GroupClient;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnConnect {
    pub jsonrpc: String,
    pub method: String,
    pub params: ClientOnConnectParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnConnectParams {
    pub id: String,
    pub client: GroupClient,
}