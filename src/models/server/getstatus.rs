use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapcastStatus {
    pub id: String,
    pub jsonrpc: String,
    pub result: ResultData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultData {
    pub server: ServerInfo,
    pub groups: Vec<Group>,
    pub streams: Vec<Stream>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub host: HostInfo,
    pub snapserver: SnapserverInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostInfo {
    pub arch: String,
    pub ip: String,
    pub mac: String,
    pub name: String,
    pub os: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapserverInfo {
    pub control_protocol_version: u32,
    pub name: String,
    pub protocol_version: u32,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: String,
    pub muted: bool,
    pub name: String,
    pub stream_id: String,
    pub clients: Vec<Client>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: String,
    pub connected: bool,
    pub config: ClientConfig,
    pub host: HostInfo,
    pub last_seen: LastSeen,
    pub snapclient: SnapclientInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientConfig {
    pub instance: u32,
    pub latency: i32,
    pub name: String,
    pub volume: Volume,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Volume {
    pub muted: bool,
    pub percent: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastSeen {
    pub sec: u64,
    pub usec: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapclientInfo {
    pub name: String,
    pub protocol_version: u32,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stream {
    pub id: String,
    pub status: String,
    pub uri: Uri,
    pub properties: StreamProperties,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Uri {
    pub fragment: String,
    pub host: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub raw: String,
    pub scheme: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamProperties {
    pub can_control: bool,
    pub can_go_next: bool,
    pub can_go_previous: bool,
    pub can_pause: bool,
    pub can_play: bool,
    pub can_seek: bool,
}