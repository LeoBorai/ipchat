use serde::{Deserialize, Serialize};

/// Configuration for ICE servers (STUN/TURN)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceServerConfig {
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

impl IceServerConfig {
    pub fn new(urls: Vec<String>) -> Self {
        Self {
            urls,
            username: None,
            credential: None,
        }
    }

    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
    }

    pub fn set_credential(&mut self, credential: Option<String>) {
        self.credential = credential;
    }
}

/// Configuration for the IPChatWebRTC client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPChatWebRTCConfig {
    pub signaling_server: String,
    pub ice_servers: Vec<IceServerConfig>,
    pub room_id: Option<String>,
}

impl IPChatWebRTCConfig {
    pub fn new(signaling_server: String) -> Self {
        Self {
            signaling_server,
            ice_servers: vec![IceServerConfig {
                urls: vec!["stun:stun.l.google.com:19302".to_string()],
                username: None,
                credential: None,
            }],
            room_id: None,
        }
    }

    pub fn set_room_id(&mut self, room_id: Option<String>) {
        self.room_id = room_id;
    }
}

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub os: String,
    pub browser: String,
}

/// File metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub last_modified: f64,
}

impl FileMetadata {
    pub fn new(name: String, size: u64, mime_type: String, last_modified: f64) -> Self {
        Self {
            name,
            size,
            mime_type,
            last_modified,
        }
    }
}

/// Transfer status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    Accepted,
    Declined,
    Transferring,
    Completed,
    Failed,
    Cancelled,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}
