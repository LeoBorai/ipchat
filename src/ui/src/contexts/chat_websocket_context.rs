use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use leptos::logging::error;
use leptos::prelude::{RwSignal, Set, Update};
use serde::{Deserialize, Serialize};

use crate::services::chat_websocket_service::{
    ChatWebSocketService, ConnectionStatus, ServerMessage,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    pub ip: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ChatWebSocketContext {
    pub is_connected: RwSignal<bool>,
    pub connection_status: RwSignal<ConnectionStatus>,
    pub my_rooms: RwSignal<Vec<Room>>,
    pub discovered_peers: RwSignal<Vec<Peer>>,
    pub active_room: RwSignal<Option<Room>>,
    pub messages: RwSignal<HashMap<String, Vec<Message>>>,
    // peer_list_interval: StoredValue<Option<Interval>>,
}

impl Default for ChatWebSocketContext {
    fn default() -> Self {
        Self {
            is_connected: RwSignal::new(false),
            connection_status: RwSignal::new(ConnectionStatus::Disconnected),
            my_rooms: RwSignal::new(Vec::new()),
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
                if let Ok(room) = serde_json::from_value::<Room>(room) {
                    self.my_rooms.update(|rooms| rooms.push(room.clone()));
                    self.messages.update(|msgs| {
                        msgs.insert(room.id.clone(), Vec::new());
                    });
                }
            }

            ServerMessage::RoomJoined { room } => {
                if let Ok(room) = serde_json::from_value::<Room>(room) {
                    self.active_room.set(Some(room.clone()));

                    self.messages.update(|msgs| {
                        if !msgs.contains_key(&room.id) {
                            let system_message = Message {
                                sender: "System".to_string(),
                                content: format!("You joined {}", room.name),
                                timestamp: Utc::now().to_rfc3339(), // FIXME: this should come from server
                                room_id: Some(room.id.clone()),
                            };
                            msgs.insert(room.id.clone(), vec![system_message]);
                        }
                    });
                }
            }

            ServerMessage::NewMessage { message } => {
                if let Ok(ref msg) = serde_json::from_value::<Message>(message) {
                    if let Some(room_id) = &msg.room_id {
                        self.messages.update(|msgs| {
                            msgs.entry(room_id.clone())
                                .or_insert_with(Vec::new)
                                .push(msg.to_owned());
                        });
                    }
                }
            }

            ServerMessage::PeerList { peers } => {
                if let Ok(peer_list) =
                    serde_json::from_value::<Vec<Peer>>(serde_json::Value::Array(peers))
                {
                    self.discovered_peers.set(peer_list);
                }
            }

            ServerMessage::RoomList { rooms } => {
                if let Ok(room_list) =
                    serde_json::from_value::<Vec<Room>>(serde_json::Value::Array(rooms.clone()))
                {
                    self.my_rooms.set(room_list.clone());

                    self.messages.update(|msgs| {
                        for room in room_list {
                            msgs.entry(room.id.clone()).or_insert_with(Vec::new);
                        }
                    });
                }
            }

            ServerMessage::Error { message } => {
                error!("Server error: {}", message);
            }
        }
    }

    pub fn connect(&self, server_url: String) -> Result<()> {
        let ctx = self.clone();
        let mut chat_web_socket_service = ChatWebSocketService::get();
        chat_web_socket_service.set_server_url(server_url);

        let is_connected = self.is_connected;
        let connection_status = self.connection_status;

        chat_web_socket_service.connect(
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

        // Set up peer list polling when connected
        self.setup_peer_list_polling();

        Ok(())
    }

    fn setup_peer_list_polling(&self) {
        // let is_connected = self.is_connected;
        // let ws_service = self.ws_service;

        // leptos::task::spawn_local(async move {
        //     loop {
        //         gloo_timers::future::TimeoutFuture::new(100).await;

        //         if is_connected.get() {
        //             // Clear any existing interval
        //             if let Some(service) = ws_service.get_value() {
        //                 // Set up 5-second interval for peer list requests
        //                 let interval = Interval::new(5000, move || {
        //                     if let Some(service) = ws_service.get_value() {
        //                         service.list_nodes();
        //                     }
        //                 });

        //                 // Note: In a real implementation, you'd want to store and manage
        //                 // the interval to clean it up when disconnecting
        //                 std::mem::forget(interval);
        //             }
        //             break;
        //         }
        //     }
        // });
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

    pub fn create_room(&self, room_name: String) {
        // if let Some(service) = self.ws_service.get_value() {
        //     service.create_room(room_name);
        // }
    }

    pub fn join_room(&self, room_id: String, peer_ip: String) {
        // if let Some(service) = self.ws_service.get_value() {
        //     service.join_room(room_id, peer_ip);
        // }
    }

    pub fn send_message(&self, room_id: String, content: String, sender: String) {
        // if let Some(service) = self.ws_service.get_value() {
        //     service.send_message(room_id, content, sender);
        // }
    }
}

impl Drop for ChatWebSocketContext {
    fn drop(&mut self) {
        self.disconnect();
    }
}
