use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetVolumeRequest {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: SetVolumeParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetVolumeParams {
    pub id: String,
    pub volume: SetVolumeParamsVolume,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetVolumeParamsVolume {
    pub muted: String,
    pub percent: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetVolumeResponse {
    pub id: String,
    pub jsonrpc: String,
    pub result: SetVolumeResult,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetVolumeResult {
    pub volume: SetVolumeResultVolume,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetVolumeResultVolume {
    pub muted: String,
    pub percent: String,
}