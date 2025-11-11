use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRoom {
    pub id: Uuid,
    pub ip: IpAddr,
    pub name: String,
}
