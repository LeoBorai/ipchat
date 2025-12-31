use leptos::prelude::{Get, Signal, expect_context};
use uuid::Uuid;

use ipchat::proto::{ChatMessage, NodeInfo, Room};

use crate::contexts::chat_websocket_context::ChatWebSocketContext;
use crate::services::chat_websocket_service::ConnectionStatus;

pub fn use_chat_ws() -> ChatWebSocketContext {
    expect_context::<ChatWebSocketContext>()
}

pub fn use_is_connected() -> Signal<bool> {
    Signal::derive(move || use_chat_ws().is_connected.get())
}

pub fn use_connection_status() -> Signal<ConnectionStatus> {
    Signal::derive(move || use_chat_ws().connection_status.get())
}

pub fn use_rooms() -> Signal<Vec<Room>> {
    Signal::derive(move || use_chat_ws().rooms.get())
}

pub fn use_nodes() -> Signal<Vec<NodeInfo>> {
    Signal::derive(move || use_chat_ws().nodes.get())
}

pub fn use_active_room() -> Signal<Option<Room>> {
    Signal::derive(move || use_chat_ws().active_room.get())
}

pub fn use_messages() -> Signal<std::collections::HashMap<Uuid, Vec<ChatMessage>>> {
    Signal::derive(move || use_chat_ws().messages.get())
}
