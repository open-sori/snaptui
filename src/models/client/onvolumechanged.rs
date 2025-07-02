use serde::{Deserialize, Serialize};
use crate::models::server::getstatus::ClientConfigVolume;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnVolumeChanged {
    pub jsonrpc: String,
    pub method: String,
    pub params: ClientOnVolumeChangedParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOnVolumeChangedParams {
    pub id: String,
    pub volume: ClientConfigVolume,
}