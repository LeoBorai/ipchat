use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRoom {
    pub id: Uuid,
    pub ip: IpAddr,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub ip: IpAddr,
    pub username: String,
    pub rooms: Vec<PeerRoom>,
    // pub last_seen: std::time::Instant,
}
