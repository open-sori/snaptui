use uuid::Uuid;
use serde_json::json;
use crate::models::group::setstream::{SetStreamRequest, SetStreamParams};

pub fn create_set_stream_request(group_id: &str, stream_id: &str) -> String {
    let uuid = Uuid::new_v4();

    let request = SetStreamRequest {
        id: uuid.to_string(),
        jsonrpc: "2.0".to_string(),
        method: "Group.SetStream".to_string(),
        params: SetStreamParams {
            id: group_id.to_string(),
            stream_id: stream_id.to_string(),
        },
    };

    json!(request).to_string()
}