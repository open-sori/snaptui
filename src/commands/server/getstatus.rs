use uuid::Uuid;
use serde_json::{json, Value, Error};
use serde::de::Error as SerdeError;
use crate::models::server::getstatus::GetStatusData;

pub fn create_status_request() -> String {
    let uuid = Uuid::new_v4();
    json!({
        "id": uuid.to_string(),
        "jsonrpc": "2.0",
        "method": "Server.GetStatus"
    }).to_string()
}

pub fn parse_status_response(response: &str) -> Result<GetStatusData, Error> {
    // First try to parse as GetStatusData
    match serde_json::from_str::<GetStatusData>(response) {
        Ok(status) => Ok(status),
        Err(e) => {
            // If parsing as GetStatusData fails, check if it's valid JSON
            if serde_json::from_str::<Value>(response).is_ok() {
                // It's valid JSON but not matching our expected format
                Err(Error::custom(format!(
                    "Valid JSON but not matching expected GetStatusData format. Error: {}",
                    e
                )))
            } else {
                // It's not valid JSON at all
                Err(Error::custom(format!("Invalid JSON: {}", e)))
            }
        }
    }
}

pub fn extract_server_version(response: &str) -> Option<String> {
    // First try to parse as GetStatusData to get the version
    if let Ok(parsed) = serde_json::from_str::<GetStatusData>(response) {
        return Some(parsed.result.server.server.snapserver.version);
    }

    // Fallback to manual extraction if the above fails
    let parsed: Value = match serde_json::from_str(response) {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };

    // Try multiple paths to find the version
    if let Some(version) = parsed.get("result")
        .and_then(|result| result.get("server"))
        .and_then(|server| server.get("snapserver"))
        .and_then(|snapserver| snapserver.get("version"))
        .and_then(|version| version.as_str().map(|s| s.to_string()))
    {
        return Some(version);
    }

    // If still not found, try alternative paths
    if let Some(version) = parsed.get("result")
        .and_then(|result| result.get("server"))
        .and_then(|server| server.get("version"))
        .and_then(|version| version.as_str().map(|s| s.to_string()))
    {
        return Some(version);
    }

    None
}