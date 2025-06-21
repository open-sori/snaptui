use uuid::Uuid;
use serde_json::json;
use crate::models::status::SnapcastStatus;
use serde_json::Value;

pub fn create_status_request() -> String {
    let uuid = Uuid::new_v4();
    json!({
        "id": uuid.to_string(),
        "jsonrpc": "2.0",
        "method": "Server.GetStatus"
    }).to_string()
}

pub fn parse_status_response(response: &str) -> Result<SnapcastStatus, serde_json::Error> {
    serde_json::from_str(response)
}

pub fn extract_server_version(response: &str) -> Option<String> {
    if let Ok(parsed) = serde_json::from_str::<Value>(response) {
        if let Some(result) = parsed.get("result") {
            if let Some(server) = result.get("server") {
                if let Some(snapserver) = server.get("snapserver") {
                    if let Some(version) = snapserver.get("version") {
                        return version.as_str().map(|s| s.to_string());
                    }
                }
            }
        }
    }
    None
}
