use serde::{Deserialize, Serialize};

/// Root structure of the Snapcast status response
/// Path: (root)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetStatusData {
    /// JSON-RPC ID field
    /// Path: .id
    pub id: String,
    /// JSON-RPC version
    /// Path: .jsonrpc
    pub jsonrpc: String,
    /// Main result data
    /// Path: .result
    pub result: Result,
}

/// Container for the result data
/// Path: .result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Result {
    /// Contains all server-related data
    /// Path: .result.server
    pub server: Server,
}

/// Container for all server-related data
/// Path: .result.server
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    /// List of client groups
    /// Path: .result.server.groups
    #[serde(default)]
    pub groups: Vec<GroupData>,
    /// Server information (host and snapserver details)
    /// Path: .result.server.server
    #[serde(default)]
    pub server: ServerData,
    /// List of available streams
    /// Path: .result.server.streams
    #[serde(default)]
    pub streams: Vec<StreamData>,
}

/// Client group information
/// Path: .result.server.groups[]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GroupData {
    /// List of clients in this group
    /// Path: .result.server.groups[].clients
    #[serde(default)]
    pub clients: Vec<GroupClient>,
    /// Group ID
    /// Path: .result.server.groups[].id
    #[serde(default)]
    pub id: String,
    /// Group mute status
    /// Path: .result.server.groups[].muted
    #[serde(default)]
    pub muted: bool,
    /// Group name
    /// Path: .result.server.groups[].name
    #[serde(default)]
    pub name: String,
    /// Stream ID associated with this group
    /// Path: .result.server.groups[].stream_id
    #[serde(default)]
    pub stream_id: String,
}

/// Client information
/// Path: .result.server.groups[].clients[]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GroupClient {
    /// Client configuration
    /// Path: .result.server.groups[].clients[].config
    #[serde(default)]
    pub config: ClientConfig,
    /// Connection status
    /// Path: .result.server.groups[].clients[].connected
    #[serde(default)]
    pub connected: bool,
    /// Host information
    /// Path: .result.server.groups[].clients[].host
    #[serde(default)]
    pub host: ClientHost,
    /// Client ID
    /// Path: .result.server.groups[].clients[].id
    #[serde(default)]
    pub id: String,
    /// Last seen timestamp
    /// Path: .result.server.groups[].clients[].last_seen
    #[serde(rename = "lastSeen", default)]
    pub last_seen: ClientLastSeen,
    /// Snapclient information
    /// Path: .result.server.groups[].clients[].snapclient
    #[serde(default)]
    pub snapclient: ClientSnapclient,
}

/// Client configuration
/// Path: .result.server.groups[].clients[].config
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClientConfig {
    /// Instance number
    /// Path: .result.server.groups[].clients[].config.instance
    #[serde(default)]
    pub instance: u32,
    /// Audio latency
    /// Path: .result.server.groups[].clients[].config.latency
    #[serde(default)]
    pub latency: i32,
    /// Client name
    /// Path: .result.server.groups[].clients[].config.name
    #[serde(default)]
    pub name: String,
    /// Volume settings
    /// Path: .result.server.groups[].clients[].config.volume
    #[serde(default)]
    pub volume: ClientConfigVolume,
}

/// Volume settings
/// Path: .result.server.groups[].clients[].config.volume
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClientConfigVolume {
    /// Mute status
    /// Path: .result.server.groups[].clients[].config.volume.muted
    #[serde(default)]
    pub muted: bool,
    /// Volume percentage
    /// Path: .result.server.groups[].clients[].config.volume.percent
    #[serde(default)]
    pub percent: u32,
}

/// Host system information
/// Path: .result.server.server.host
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClientHost {
    /// Host architecture
    /// Path: .result.server.server.host.arch
    #[serde(default)]
    pub arch: String,
    /// Host IP address
    /// Path: .result.server.server.host.ip
    #[serde(default)]
    pub ip: String,
    /// Host MAC address
    /// Path: .result.server.server.host.mac
    #[serde(default)]
    pub mac: String,
    /// Host name
    /// Path: .result.server.server.host.name
    #[serde(default)]
    pub name: String,
    /// Host operating system
    /// Path: .result.server.server.host.os
    #[serde(default)]
    pub os: String,
}

/// Last seen timestamp
/// Path: .result.server.groups[].clients[].lastSeen
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClientLastSeen {
    /// Seconds since epoch
    /// Path: .result.server.groups[].clients[].lastSeen.sec
    #[serde(default)]
    pub sec: u64,
    /// Microseconds
    /// Path: .result.server.groups[].clients[].lastSeen.usec
    #[serde(default)]
    pub usec: u64,
}

/// Snapclient information
/// Path: .result.server.groups[].clients[].snapclient
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClientSnapclient {
    /// Client name
    /// Path: .result.server.groups[].clients[].snapclient.name
    #[serde(default)]
    pub name: String,
    /// Protocol version
    /// Path: .result.server.groups[].clients[].snapclient.protocolVersion
    #[serde(rename = "protocolVersion", default)]
    pub protocol_version: u32,
    /// Client version
    /// Path: .result.server.groups[].clients[].snapclient.version
    #[serde(default)]
    pub version: String,
}

/// Server information including host and snapserver details
/// Path: .result.server.server
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServerData {
    /// Host system information
    /// Path: .result.server.server.host
    #[serde(default)]
    pub host: ServerHost,
    /// Snapserver information
    /// Path: .result.server.server.snapserver
    #[serde(default)]
    pub snapserver: ServerSnapserver,
}

/// Host information
/// Path: .result.server.server.host
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServerHost {
    /// Host architecture
    /// Path: .result.server.server.host.arch
    #[serde(default)]
    pub arch: String,
    /// Host IP address
    /// Path: .result.server.server.host.ip
    #[serde(default)]
    pub ip: String,
    /// Host MAC address
    /// Path: .result.server.server.host.mac
    #[serde(default)]
    pub mac: String,
    /// Host name
    /// Path: .result.server.server.host.name
    #[serde(default)]
    pub name: String,
    /// Host operating system
    /// Path: .result.server.server.host.os
    #[serde(default)]
    pub os: String,
}

/// Snapserver information
/// Path: .result.server.server.snapserver
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServerSnapserver {
    /// Control protocol version
    /// Path: .result.server.server.snapserver.controlProtocolVersion
    #[serde(rename = "controlProtocolVersion", default)]
    pub control_protocol_version: u32,
    /// Snapserver name
    /// Path: .result.server.server.snapserver.name
    #[serde(default)]
    pub name: String,
    /// Protocol version
    /// Path: .result.server.server.snapserver.protocolVersion
    #[serde(rename = "protocolVersion", default)]
    pub protocol_version: u32,
    /// Snapserver version
    /// Path: .result.server.server.snapserver.version
    #[serde(default)]
    pub version: String,
}

/// Stream information
/// Path: .result.server.streams[]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StreamData {
    /// Stream ID
    /// Path: .result.server.streams[].id
    #[serde(default)]
    pub id: String,
    /// Stream properties
    /// Path: .result.server.streams[].properties
    #[serde(default)]
    pub properties: StreamProperties,
    /// Stream status
    /// Path: .result.server.streams[].status
    #[serde(default)]
    pub status: String,
    /// Stream URI
    /// Path: .result.server.streams[].uri
    #[serde(default)]
    pub uri: StreamUri,
}

/// Stream properties
/// Path: .result.server.streams[].properties
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StreamProperties {
    /// Can control the stream
    /// Path: .result.server.streams[].properties.canControl
    #[serde(rename = "canControl", default)]
    pub can_control: bool,
    /// Can go to next track
    /// Path: .result.server.streams[].properties.canGoNext
    #[serde(rename = "canGoNext", default)]
    pub can_go_next: bool,
    /// Can go to previous track
    /// Path: .result.server.streams[].properties.canGoPrevious
    #[serde(rename = "canGoPrevious", default)]
    pub can_go_previous: bool,
    /// Can pause the stream
    /// Path: .result.server.streams[].properties.canPause
    #[serde(rename = "canPause", default)]
    pub can_pause: bool,
    /// Can play the stream
    /// Path: .result.server.streams[].properties.canPlay
    #[serde(rename = "canPlay", default)]
    pub can_play: bool,
    /// Can seek in the stream
    /// Path: .result.server.streams[].properties.canSeek
    #[serde(rename = "canSeek", default)]
    pub can_seek: bool,
}

/// Stream URI information
/// Path: .result.server.streams[].uri
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StreamUri {
    /// URI fragment
    /// Path: .result.server.streams[].uri.fragment
    #[serde(default)]
    pub fragment: String,
    /// URI host
    /// Path: .result.server.streams[].uri.host
    #[serde(default)]
    pub host: String,
    /// URI path
    /// Path: .result.server.streams[].uri.path
    #[serde(default)]
    pub path: String,
    /// Query information
    /// Path: .result.server.streams[].uri.query
    #[serde(default)]
    pub query: UriQuery,
    /// Raw URI string
    /// Path: .result.server.streams[].uri.raw
    #[serde(default)]
    pub raw: String,
    /// URI scheme
    /// Path: .result.server.streams[].uri.scheme
    #[serde(default)]
    pub scheme: String,
}

/// Query properties
/// Path: .result.server.streams[].uri.query
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UriQuery {
    /// Chunk size in milliseconds
    /// Path: .result.server.streams[].uri.query.chunk_ms
    #[serde(default)]
    pub chunk_ms: String,
    /// Codec name
    /// Path: .result.server.streams[].uri.query.codec
    #[serde(default)]
    pub codec: String,
    /// Stream mode
    /// Path: .result.server.streams[].uri.query.mode
    #[serde(default)]
    pub mode: String,
    /// Stream name
    /// Path: .result.server.streams[].uri.query.name
    #[serde(default)]
    pub name: String,
    /// Sample format
    /// Path: .result.server.streams[].uri.query.sampleformat
    #[serde(default)]
    pub sampleformat: String,
}