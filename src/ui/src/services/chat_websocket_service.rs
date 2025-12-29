use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Result, bail};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::Message;
use gloo_net::websocket::futures::WebSocket;
use gloo_timers::future::TimeoutFuture;
use leptos::logging::{error, log};
use serde::{Deserialize, Serialize};
use serde_json::Value;

thread_local!(pub static CHAT_WEB_SOCKET_SERVICE: RefCell<ChatWebSocketService> = RefCell::new(ChatWebSocketService::default()));

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    RoomCreated { room: Value },
    RoomJoined { room: Value },
    NewMessage { message: Value },
    PeerList { peers: Vec<Value> },
    RoomList { rooms: Vec<Value> },
    Error { message: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
    Reconnecting,
    ConnectionFailed,
}

impl std::fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::Connecting => write!(f, "Connecting..."),
            ConnectionStatus::Connected => write!(f, "Connected"),
            ConnectionStatus::Error => write!(f, "Error"),
            ConnectionStatus::Reconnecting => write!(f, "Reconnecting..."),
            ConnectionStatus::ConnectionFailed => write!(f, "Connection Failed"),
        }
    }
}

pub struct ChatWebSocketConfig<F1, F2, F3>
where
    F1: Fn(bool, ConnectionStatus) + 'static,
    F2: Fn(ServerMessage) + 'static,
    F3: Fn() + 'static,
{
    pub server_url: String,
    pub reconnect_delay: Duration,
    pub on_connection_change: Option<F1>,
    pub on_message: Option<F2>,
    pub on_initial_connect: Option<F3>,
}

impl<F1, F2, F3> ChatWebSocketConfig<F1, F2, F3>
where
    F1: Fn(bool, ConnectionStatus) + 'static,
    F2: Fn(ServerMessage) + 'static,
    F3: Fn() + 'static,
{
    pub fn new(server_url: String) -> Self {
        Self {
            server_url,
            reconnect_delay: Duration::from_millis(3000),
            on_connection_change: None,
            on_message: None,
            on_initial_connect: None,
        }
    }

    pub fn with_reconnect_delay(mut self, delay: Duration) -> Self {
        self.reconnect_delay = delay;
        self
    }

    pub fn with_connection_change(mut self, callback: F1) -> Self {
        self.on_connection_change = Some(callback);
        self
    }

    pub fn with_message(mut self, callback: F2) -> Self {
        self.on_message = Some(callback);
        self
    }

    pub fn with_initial_connect(mut self, callback: F3) -> Self {
        self.on_initial_connect = Some(callback);
        self
    }
}

#[derive(Clone)]
pub struct ChatWebSocketService {
    ws: Option<Arc<Mutex<WebSocket>>>,
    server_url: Option<String>,
    reconnect_delay: Duration,
    should_reconnect: bool,
    is_connected: bool,
}

impl std::fmt::Debug for ChatWebSocketService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChatWebSocketService")
            .field("server_url", &self.server_url)
            .field("reconnect_delay", &self.reconnect_delay)
            .field("should_reconnect", &self.should_reconnect)
            .field("is_connected", &self.is_connected)
            .finish()
    }
}

impl Default for ChatWebSocketService {
    fn default() -> Self {
        Self {
            ws: None,
            server_url: None,
            reconnect_delay: Duration::from_millis(3000),
            should_reconnect: false,
            is_connected: false,
        }
    }
}

impl ChatWebSocketService {
    pub fn get() -> ChatWebSocketService {
        CHAT_WEB_SOCKET_SERVICE.with(|service| service.borrow().clone())
    }

    pub fn set_server_url(&mut self, server_url: String) {
        self.server_url = Some(server_url);
    }

    pub fn set_reconnect_delay(&mut self, delay: Duration) {
        self.reconnect_delay = delay;
    }

    pub fn set_should_reconnect(&mut self, should_reconnect: bool) {
        self.should_reconnect = should_reconnect;
    }

    pub fn connect<F1, F2, F3>(
        &mut self,
        on_connection_change: F1,
        on_message: F2,
        on_initial_connect: F3,
    ) where
        F1: Fn(bool, ConnectionStatus) + Clone + 'static,
        F2: Fn(ServerMessage) + Clone + 'static,
        F3: Fn() + Clone + 'static,
    {
        self.should_reconnect = true;
        self.create_connection(on_connection_change, on_message, on_initial_connect);
    }

    fn create_connection<F1, F2, F3>(
        &mut self,
        on_connection_change: F1,
        on_message: F2,
        on_initial_connect: F3,
    ) -> Result<()>
    where
        F1: Fn(bool, ConnectionStatus) + Clone + 'static,
        F2: Fn(ServerMessage) + Clone + 'static,
        F3: Fn() + Clone + 'static,
    {
        let Some(server_url) = &self.server_url else {
            bail!("Server URL is not set");
        };

        log!("Connecting to server: {:?}", self.server_url);

        on_connection_change(false, ConnectionStatus::Connecting);

        match WebSocket::open(&server_url) {
            Ok(ws) => {
                log!("Connected to server");
                on_connection_change(true, ConnectionStatus::Connected);
                on_initial_connect();

                self.is_connected = true;

                let (_, mut receiver) = ws.split();
                let should_reconnect = self.should_reconnect;
                let reconnect_delay = self.reconnect_delay;
                let on_connection_change_clone = on_connection_change.clone();
                let on_message_clone = on_message.clone();

                leptos::task::spawn_local(async move {
                    while let Some(msg) = receiver.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                match serde_json::from_str::<ServerMessage>(&text) {
                                    Ok(message) => {
                                        log!("Received: {:?}", message);
                                        on_message_clone(message);
                                    }
                                    Err(e) => {
                                        error!("Failed to parse message: {:?}", e);
                                    }
                                }
                            }
                            Ok(Message::Bytes(_)) => {
                                log!("Received binary message (not supported)");
                            }
                            Err(e) => {
                                error!("WebSocket error: {:?}", e);
                                on_connection_change_clone(false, ConnectionStatus::Error);
                                break;
                            }
                        }
                    }

                    // Connection closed
                    log!("Disconnected from server");
                    on_connection_change_clone(false, ConnectionStatus::Disconnected);

                    if should_reconnect {
                        on_connection_change_clone(false, ConnectionStatus::Reconnecting);
                        TimeoutFuture::new(reconnect_delay.as_millis() as u32).await;
                    }
                });

                Ok(())
            }
            Err(e) => {
                on_connection_change(false, ConnectionStatus::ConnectionFailed);
                bail!("Failed to connect: {:?}", e)
            }
        }
    }

    pub fn disconnect(&mut self) {
        self.ws = None;
        self.should_reconnect = false;
        self.is_connected = false;
    }

    pub fn send(&self, message: &impl Serialize) {
        if let Some(ws) = &self.ws {
            let mut ws = ws.lock().unwrap(); // remove unwrap in production code
            if let Ok(json) = serde_json::to_string(message) {
                ws.send(Message::Text(json));
            }
        } else {
            error!("WebSocket is not connected");
        }
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn list_nodes(&self) {
        #[derive(Serialize)]
        struct ListNodesMessage {
            r#type: String,
        }
        self.send(&ListNodesMessage {
            r#type: "ListNodes".to_string(),
        });
    }

    pub fn list_rooms(&self) {
        #[derive(Serialize)]
        struct ListRoomsMessage {
            r#type: String,
        }
        self.send(&ListRoomsMessage {
            r#type: "ListRooms".to_string(),
        });
    }

    pub fn create_room(&self, name: String) {
        #[derive(Serialize)]
        struct CreateRoomMessage {
            r#type: String,
            name: String,
        }
        self.send(&CreateRoomMessage {
            r#type: "CreateRoom".to_string(),
            name,
        });
    }

    pub fn join_room(&self, room_id: String, peer_ip: String) {
        #[derive(Serialize)]
        struct JoinRoomMessage {
            r#type: String,
            room_id: String,
            peer_ip: String,
        }
        self.send(&JoinRoomMessage {
            r#type: "JoinRoom".to_string(),
            room_id,
            peer_ip,
        });
    }

    pub fn send_message(&self, room_id: String, content: String, sender: String) {
        #[derive(Serialize)]
        struct SendMessageMessage {
            r#type: String,
            room_id: String,
            content: String,
            sender: String,
        }
        self.send(&SendMessageMessage {
            r#type: "SendMessage".to_string(),
            room_id,
            content,
            sender,
        });
    }
}
