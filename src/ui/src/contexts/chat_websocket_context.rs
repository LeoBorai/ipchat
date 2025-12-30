use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use gloo_timers::callback::Interval;
use gloo_timers::future::TimeoutFuture;
use leptos::logging::{error, log};
use leptos::prelude::{RwSignal, Set, Update};
use uuid::Uuid;

use ipchat::proto::{ChatMessage, PeerRoom, Room, ServerMessage};

use crate::services::chat_websocket_service::{ChatWebSocketService, ConnectionStatus};

#[derive(Clone, Debug)]
pub struct ChatWebSocketContext {
    pub is_connected: RwSignal<bool>,
    pub connection_status: RwSignal<ConnectionStatus>,
    pub rooms: RwSignal<Vec<Room>>,
    pub discovered_peers: RwSignal<Vec<PeerRoom>>,
    pub active_room: RwSignal<Option<Room>>,
    pub messages: RwSignal<HashMap<Uuid, Vec<ChatMessage>>>,
    // peer_list_interval: StoredValue<Option<Interval>>,
}

impl Default for ChatWebSocketContext {
    fn default() -> Self {
        Self {
            is_connected: RwSignal::new(false),
            connection_status: RwSignal::new(ConnectionStatus::Disconnected),
            rooms: RwSignal::new(Vec::new()),
            discovered_peers: RwSignal::new(Vec::new()),
            active_room: RwSignal::new(None),
            messages: RwSignal::new(HashMap::new()),
            // peer_list_interval: StoredValue::new(None),
        }
    }
}

impl ChatWebSocketContext {
    fn handle_server_message(&self, message: ServerMessage) {
        match message {
            ServerMessage::RoomCreated { room } => {
                self.rooms.update(|rooms| rooms.push(room.clone()));
                self.messages.update(|msgs| {
                    msgs.insert(room.id, Vec::new());
                });
            }
            ServerMessage::RoomJoined { room } => {
                self.active_room.set(Some(room.clone()));

                self.messages.update(|msgs| {
                    if let std::collections::hash_map::Entry::Vacant(e) = msgs.entry(room.id) {
                        let system_message = ChatMessage {
                            sender: "System".to_string(),
                            content: format!("You joined {}", room.name),
                            timestamp: Utc::now(),
                            room_id: room.id,
                        };
                        e.insert(vec![system_message]);
                    }
                });
            }
            ServerMessage::NewMessage { message } => {
                self.messages.update(|msgs| {
                    msgs.entry(message.room_id)
                        .or_insert_with(Vec::new)
                        .push(message.to_owned());
                });
            }
            ServerMessage::RoomList { rooms } => {
                self.rooms.set(rooms.clone());
                self.messages.update(|msgs| {
                    for room in rooms {
                        msgs.entry(room.id).or_insert_with(Vec::new);
                    }
                });
            }
            ServerMessage::Error { message } => {
                error!("Server error: {}", message);
            }
            _ => {
                log!("Unhandled server message: {:?}", message);
            }
        }
    }

    pub fn connect(&self, server_url: String) -> Result<()> {
        let ctx = self.clone();
        let chat_web_socket_service = ChatWebSocketService::get();
        chat_web_socket_service
            .borrow_mut()
            .set_server_url(server_url);

        let is_connected = self.is_connected;
        let connection_status = self.connection_status;

        chat_web_socket_service.borrow_mut().connect(
            move |connected, status| {
                is_connected.set(connected);
                connection_status.set(status);
            },
            move |message| {
                ctx.handle_server_message(message);
            },
            {
                move |ws_service| {
                    let ws_service = ws_service.clone();
                    leptos::task::spawn_local(async move {
                        ws_service.list_nodes().await;
                        ws_service.list_rooms().await;
                    });
                }
            },
        )?;

        self.setup_peer_list_polling();

        Ok(())
    }

    fn setup_peer_list_polling(&self) {
        leptos::task::spawn_local(async move {
            loop {
                TimeoutFuture::new(1000).await;

                let ws_service = ChatWebSocketService::get();

                if ws_service.borrow().is_connected() {
                    let interval = Interval::new(1000, move || {
                        let ws_service = ChatWebSocketService::get();
                        leptos::task::spawn_local(async move {
                            ws_service.borrow_mut().list_nodes().await;
                        });
                    });

                    // do not drop it (yet)
                    std::mem::forget(interval);

                    break;
                }
            }
        });
    }

    pub fn disconnect(&self) {
        // Clean up interval
        // if let Some(interval) = self.peer_list_interval.get_value() {
        //     interval.cancel();
        // }
        // self.peer_list_interval.set_value(None);

        // if let Some(service) = self.ws_service.get_value() {
        //     service.disconnect();
        // }
        // self.ws_service.set_value(None);
    }

    pub fn set_active_room(&self, room: Option<Room>) {
        self.active_room.set(room);
    }

    pub fn request_peer_list(&self) {
        // if let Some(service) = self.ws_service.get_value() {
        //     service.list_nodes();
        // }
    }

    pub fn request_room_list(&self) {
        // if let Some(service) = self.ws_service.get_value() {
        //     service.list_rooms();
        // }
    }

    pub async fn create_room(&self, room_name: String) -> Result<()> {
        let chat_web_socket_service = ChatWebSocketService::get();
        chat_web_socket_service
            .borrow()
            .create_room(room_name)
            .await
    }

    pub fn join_room(&self, _room_id: String, _peer_ip: String) {
        // if let Some(service) = self.ws_service.get_value() {
        //     service.join_room(room_id, peer_ip);
        // }
    }

    pub async fn send_message(&self, room_id: Uuid, content: String, sender: String) -> Result<()> {
        let chat_web_socket_service = ChatWebSocketService::get();
        chat_web_socket_service
            .borrow()
            .send_message(room_id, content, sender)
            .await
    }
}

impl Drop for ChatWebSocketContext {
    fn drop(&mut self) {
        self.disconnect();
    }
}
