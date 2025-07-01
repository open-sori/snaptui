use uuid::Uuid;
use serde_json::json;
use crate::models::client::setlatency::{SetLatencyRequest, SetLatencyParams};

pub fn create_set_latency_request(client_id: &str, new_latency: i32) -> String {
    let uuid = Uuid::new_v4();

    let request = SetLatencyRequest {
        id: uuid.to_string(),
        jsonrpc: "2.0".to_string(),
        method: "Client.SetLatency".to_string(),
        params: SetLatencyParams {
            id: client_id.to_string(),
            latency: new_latency,
        },
    };

    json!(request).to_string()
}