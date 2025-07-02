use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupOnMute {
    pub jsonrpc: String,
    pub method: String,
    pub params: GroupOnMuteParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupOnMuteParams {
    pub id: String,
    pub mute: bool,
}