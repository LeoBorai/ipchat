use std::{
    collections::{HashSet, VecDeque},
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, mpsc},
};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};
use uuid::Uuid;

use crate::peer::{PeerRoom, SharedPeer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub ip: IpAddr,
    pub username: String,
    pub rooms: Vec<PeerRoom>,
    // pub last_seen: std::time::Instant,
}

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

pub struct ChatService {
    peer: SharedPeer,
}

impl ChatService {
    pub fn new(peer: SharedPeer) -> Self {
        Self { peer }
    }

    pub async fn start(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        let peer = self.peer.clone();

        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                let peer = Arc::clone(&peer);

                tokio::spawn(async move {
                    info!(%addr, "New chat connection established");
                    if let Err(e) = Self::handle_connection(stream, addr, peer).await {
                        error!(?e, %addr, "Error handling chat connection");
                    }
                });
            }
        });

        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        addr: SocketAddr,
        peer: SharedPeer,
    ) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (ws_tx, mut ws_rx) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

        {
            let mut state_write = peer.write().await;
            state_write.connections.insert(addr, tx);
        }

        // Task to forward messages from WebSocket to the server
        let ws_sender = Arc::new(Mutex::new(ws_tx));
        let ws_sender_clone = Arc::clone(&ws_sender);

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Ok(json) = serde_json::to_string(&msg) {
                    let ws_msg = Message::Text(json.into());
                    let mut sender = ws_sender_clone.lock().await;
                    if sender.send(ws_msg).await.is_err() {
                        break;
                    }
                }
            }
        });

        while let Some(msg) = ws_rx.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text)
                        && let Err(err) =
                            Self::handle_client_message(client_msg, addr, Arc::clone(&peer)).await
                    {
                        error!(%addr, ?err, "Failed to handle client message");
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(_) => break,
                _ => {}
            }
        }

        {
            let mut state_write = peer.write().await;
            state_write.connections.remove(&addr);
            info!(%addr, "Connection removed from peer state");
        }

        Ok(())
    }

    async fn handle_client_message(
        msg: ClientMessage,
        addr: SocketAddr,
        peer: SharedPeer,
    ) -> Result<()> {
        match msg {
            ClientMessage::CreateRoom { name } => {
                let mut peer = peer.write().await;
                let room = Room {
                    id: Uuid::new_v4(),
                    name,
                    host: peer.ip.into(),
                    participants: HashSet::from([peer.ip.into()]),
                    messages: VecDeque::new(),
                };

                peer.rooms.insert(room.id, room.clone());

                if let Some(tx) = peer.connections.get(&addr)
                    && let Err(err) = tx.send(ServerMessage::RoomCreated { room })
                {
                    error!(%addr, ?err, "Failed to send RoomCreated message");
                }

                Ok(())
            }

            ClientMessage::SendMessage { room_id, content } => {
                let mut peer = peer.write().await;
                let username = peer.username.clone();

                if let Some(room) = peer.rooms.get_mut(&room_id) {
                    let message = ChatMessage {
                        sender: username.clone(),
                        content,
                        timestamp: Utc::now(),
                        room_id,
                    };

                    room.messages.push_back(message.clone());

                    // Broadcast to all connections in the room
                    for tx in peer.connections.values() {
                        let _ = tx.send(ServerMessage::NewMessage {
                            message: message.clone(),
                        });
                    }
                }

                Ok(())
            }

            ClientMessage::ListPeers => {
                let peer = peer.read().await;
                let peers: Vec<PeerInfo> = peer.discovered_peers.values().cloned().collect();

                if let Some(tx) = peer.connections.get(&addr) {
                    let _ = tx.send(ServerMessage::PeerList { peers });
                }

                Ok(())
            }

            ClientMessage::ListRooms => {
                let state_read = peer.read().await;
                let rooms: Vec<Room> = state_read.rooms.values().cloned().collect();

                if let Some(tx) = state_read.connections.get(&addr) {
                    let _ = tx.send(ServerMessage::RoomList { rooms });
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}
