use serde::{Deserialize, Serialize};
use crate::models::server::getstatus::Server;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerOnUpdate {
    pub jsonrpc: String,
    pub method: String,
    pub params: ServerOnUpdateParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerOnUpdateParams {
    pub server: Server,
}