use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupOnNameChanged {
    pub jsonrpc: String,
    pub method: String,
    pub params: GroupOnNameChangedParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupOnNameChangedParams {
    pub id: String,
    pub name: String,
}