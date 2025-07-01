use uuid::Uuid;
use serde_json::json;
use crate::models::client::setvolume::{SetVolumeRequest, SetVolumeParams, SetVolumeParamsVolume};

pub fn create_set_volume_request(client_id: &str, client_volume_muted: &str, client_volume_percent: &str) -> String {
    let uuid = Uuid::new_v4();

    let request = SetVolumeRequest {
        id: uuid.to_string(),
        jsonrpc: "2.0".to_string(),
        method: "Client.SetVolume".to_string(),
        params: SetVolumeParams {
            id: client_id.to_string(),
            volume: SetVolumeParamsVolume {
                muted: client_volume_muted.to_string(),
                percent: client_volume_percent.to_string(),
            },
        },
    };

    json!(request).to_string()
}