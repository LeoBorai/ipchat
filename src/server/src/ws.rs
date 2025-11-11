use std::collections::{HashSet, VecDeque};
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};
use uuid::Uuid;

use crate::chat::{ChatMessage, ClientMessage, Room, ServerMessage};
use crate::node::ArcNode;
use crate::peer::PeerInfo;

/// Service in charge of handling WebSocket connections for real-time chat communication
pub struct WebSocket {
    addr: SocketAddr,
    node: ArcNode,
    tcp_listener: Arc<TcpListener>,
}

impl WebSocket {
    pub async fn new(node: ArcNode) -> Result<Self> {
        let tcp_listener = TcpListener::bind("0.0.0.0:0").await?;
        let tcp_listener = Arc::new(tcp_listener);
        let addr = tcp_listener.local_addr()?;

        Ok(Self {
            addr,
            node,
            tcp_listener,
        })
    }

    pub async fn start(&self) -> Result<()> {
        let node = Arc::clone(&self.node);
        let tcp_listener = Arc::clone(&self.tcp_listener);

        tokio::spawn(async move {
            while let Ok((stream, addr)) = tcp_listener.accept().await {
                let peer = Arc::clone(&node);

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

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    async fn handle_connection(stream: TcpStream, addr: SocketAddr, node: ArcNode) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (ws_tx, mut ws_rx) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

        {
            let mut state_write = node.write().await;
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
                            Self::handle_client_message(client_msg, addr, Arc::clone(&node)).await
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
            let mut state_write = node.write().await;
            state_write.connections.remove(&addr);
            info!(%addr, "Connection removed from peer state");
        }

        Ok(())
    }

    async fn handle_client_message(
        msg: ClientMessage,
        addr: SocketAddr,
        node: ArcNode,
    ) -> Result<()> {
        match msg {
            ClientMessage::CreateRoom { name } => {
                let mut node = node.write().await;
                let room = Room {
                    id: Uuid::new_v4(),
                    name,
                    host: node.ip.into(),
                    participants: HashSet::from([node.ip.into()]),
                    messages: VecDeque::new(),
                };

                node.rooms.insert(room.id, room.clone());

                if let Some(tx) = node.connections.get(&addr)
                    && let Err(err) = tx.send(ServerMessage::RoomCreated { room })
                {
                    error!(%addr, ?err, "Failed to send RoomCreated message");
                }

                Ok(())
            }

            ClientMessage::SendMessage { room_id, content } => {
                let mut node = node.write().await;
                let username = node.username.clone();

                if let Some(room) = node.rooms.get_mut(&room_id) {
                    let message = ChatMessage {
                        sender: username.clone(),
                        content,
                        timestamp: Utc::now(),
                        room_id,
                    };

                    room.messages.push_back(message.clone());

                    // Broadcast to all connections in the room
                    for tx in node.connections.values() {
                        let _ = tx.send(ServerMessage::NewMessage {
                            message: message.clone(),
                        });
                    }
                }

                Ok(())
            }

            ClientMessage::ListPeers => {
                let peer = node.read().await;
                let peers: Vec<PeerInfo> = peer.discovered_peers.values().cloned().collect();

                if let Some(tx) = peer.connections.get(&addr) {
                    let _ = tx.send(ServerMessage::PeerList { peers });
                }

                Ok(())
            }

            ClientMessage::ListRooms => {
                let state_read = node.read().await;
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
