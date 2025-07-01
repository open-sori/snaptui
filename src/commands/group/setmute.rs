use uuid::Uuid;
use serde_json::json;
use crate::models::group::setmute::{SetMuteRequest, SetMuteParams};

pub fn create_set_mute_request(group_id: &str, mute: bool) -> String {
    let uuid = Uuid::new_v4();

    let request = SetMuteRequest {
        id: uuid.to_string(),
        jsonrpc: "2.0".to_string(),
        method: "Group.SetMute".to_string(),
        params: SetMuteParams {
            id: group_id.to_string(),
            mute,
        },
    };

    json!(request).to_string()
}