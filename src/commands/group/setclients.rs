use uuid::Uuid;
use serde_json::json;
use crate::models::group::setclients::{SetClientsRequest, SetClientsParams};

pub fn create_set_clients_request(group_id: &str, client_ids: Vec<String>) -> String {
    let uuid = Uuid::new_v4();

    let request = SetClientsRequest {
        id: uuid.to_string(),
        jsonrpc: "2.0".to_string(),
        method: "Group.SetClients".to_string(),
        params: SetClientsParams {
            id: group_id.to_string(),
            clients: client_ids,
        },
    };

    json!(request).to_string()
}