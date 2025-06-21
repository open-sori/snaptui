use uuid::Uuid;
use serde_json::json;

pub fn create_status_request() -> String {
    let uuid = Uuid::new_v4();
    json!({
        "id": uuid.to_string(),
        "jsonrpc": "2.0",
        "method": "Server.GetStatus"
    }).to_string()
}