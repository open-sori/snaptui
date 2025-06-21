use uuid::Uuid;
use serde_json::json;
use crate::models::group::setname::{SetNameRequest, SetNameParams};

pub fn create_set_name_request(group_id: &str, new_name: &str) -> String {
    let uuid = Uuid::new_v4();

    let request = SetNameRequest {
        id: uuid.to_string(),
        jsonrpc: "2.0".to_string(),
        method: "Group.SetName".to_string(),
        params: SetNameParams {
            id: group_id.to_string(),
            name: new_name.to_string(),
        },
    };

    json!(request).to_string()
}