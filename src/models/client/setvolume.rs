use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct SetVolumeRequest {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    #[validate(nested)] // Changed from #[validate]
    pub params: SetVolumeParams,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct SetVolumeParams {
    pub id: String,
    #[validate(nested)] // Changed from #[validate]
    pub volume: SetVolumeParamsVolume,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct SetVolumeParamsVolume {
    pub muted: bool,
    #[validate(range(min = 0, max = 100))]
    pub percent: u32,
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
    pub muted: bool,
    pub percent: u32,
}