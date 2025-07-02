use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnNameChanged {
    pub jsonrpc: String,
    pub method: String,
    pub params: ClientOnNameChangedParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnNameChangedParams {
    pub id: String,
    pub name: String,
}