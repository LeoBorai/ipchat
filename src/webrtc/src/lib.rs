pub mod connection;
pub mod device;
pub mod error;
pub mod file_transfer;
pub mod message;
pub mod peer;
pub mod signaling;
pub mod types;

pub use connection::PeerConnection;
pub use device::DeviceManager;
pub use error::{IPChatWebRTCError, Result};
pub use file_transfer::{FileTransfer, FileTransferProgress};
pub use message::{Message, MessageType};
pub use peer::Peer;
pub use signaling::SignalingClient;
pub use types::IPChatWebRTCConfig;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
