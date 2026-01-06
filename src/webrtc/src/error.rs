use thiserror::Error;
use wasm_bindgen::prelude::*;

pub type Result<T> = std::result::Result<T, IPChatWebRTCError>;

#[derive(Error, Debug, Clone)]
pub enum IPChatWebRTCError {
    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("WebRTC error: {0}")]
    WebRtcError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("File transfer error: {0}")]
    FileTransferError(String),

    #[error("Invalid peer ID: {0}")]
    InvalidPeerId(String),

    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("JavaScript error: {0}")]
    JsError(String),
}

impl From<JsValue> for IPChatWebRTCError {
    fn from(err: JsValue) -> Self {
        IPChatWebRTCError::JsError(err.as_string().unwrap_or_else(|| format!("{:?}", err)))
    }
}

impl From<serde_json::Error> for IPChatWebRTCError {
    fn from(err: serde_json::Error) -> Self {
        IPChatWebRTCError::SerializationError(err.to_string())
    }
}

impl IPChatWebRTCError {
    pub fn message(&self) -> String {
        self.to_string()
    }
}
