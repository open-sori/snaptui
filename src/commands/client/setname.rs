use uuid::Uuid;
use serde_json::json;
use crate::models::client::setname::{SetNameRequest, SetNameParams};

pub fn create_set_name_request(client_id: &str, new_name: &str) -> String {
    let uuid = Uuid::new_v4();

    let request = SetNameRequest {
        id: uuid.to_string(),
        jsonrpc: "2.0".to_string(),
        method: "Client.SetName".to_string(),
        params: SetNameParams {
            id: client_id.to_string(),
            name: new_name.to_string(),
        },
    };

    json!(request).to_string()
}