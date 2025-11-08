use std::collections::{HashSet, VecDeque};
use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::peer::PeerInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub room_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub host: IpAddr,
    pub participants: HashSet<IpAddr>,
    #[serde(skip)]
    pub messages: VecDeque<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    CreateRoom { name: String },
    JoinRoom { room_id: Uuid, peer_ip: IpAddr },
    LeaveRoom { room_id: Uuid },
    Register { username: String },
    SendMessage { room_id: Uuid, content: String },
    ListPeers,
    ListRooms,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    RoomCreated { room: Room },
    RoomJoined { room: Room },
    RoomLeft { room_id: Uuid },
    NewMessage { message: ChatMessage },
    PeerList { peers: Vec<PeerInfo> },
    RoomList { rooms: Vec<Room> },
    Error { message: String },
}
