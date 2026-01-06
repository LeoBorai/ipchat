use crate::types::FileMetadata;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Message types for peer-to-peer communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MessageType {
    /// Text message
    Text { content: String },

    /// File transfer request
    FileOffer {
        files: Vec<FileMetadata>,
        transfer_id: String,
    },

    /// Response to file offer
    FileResponse { transfer_id: String, accepted: bool },

    /// File chunk data
    FileChunk {
        transfer_id: String,
        file_index: usize,
        chunk_index: usize,
        data: Vec<u8>,
    },

    /// File transfer progress update
    FileProgress {
        transfer_id: String,
        bytes_transferred: u64,
        total_bytes: u64,
    },

    /// File transfer complete
    FileComplete { transfer_id: String },

    /// Ping/keepalive
    Ping,

    /// Pong response
    Pong,
}

/// Wrapper for messages sent between peers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub timestamp: f64,
    pub payload: MessageType,
}

impl Message {
    /// Create a text message
    pub fn text(from: String, to: String, content: String) -> Self {
        Self {
            from,
            to,
            timestamp: js_sys::Date::now(),
            payload: MessageType::Text { content },
        }
    }

    /// Get message as JSON string
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(self).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Parse message from JSON string
    pub fn from_json(json: &str) -> Result<Message, JsValue> {
        serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// Signaling messages for WebRTC connection establishment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SignalingMessage {
    /// Register with the signaling server
    Register {
        device_name: String,
        room_id: Option<String>,
    },

    /// Server assigns a peer ID
    Registered { peer_id: String },

    /// Notify about peers
    PeersUpdate { peers: Vec<PeerInfo> },

    /// WebRTC offer
    Offer {
        from: String,
        to: String,
        sdp: String,
    },

    /// WebRTC answer
    Answer {
        from: String,
        to: String,
        sdp: String,
    },

    /// ICE candidate
    IceCandidate {
        from: String,
        to: String,
        candidate: String,
        sdp_mid: Option<String>,
        sdp_m_line_index: Option<u16>,
    },

    /// Peer disconnected
    PeerDisconnected { peer_id: String },

    /// Error message
    Error { message: String },

    /// Heartbeat
    Ping,

    /// Heartbeat response
    Pong,
}

/// Information about a peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub name: String,
    pub device_type: String,
}

impl PeerInfo {
    pub fn new(id: String, name: String, device_type: String) -> Self {
        Self {
            id,
            name,
            device_type,
        }
    }
}

impl SignalingMessage {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
