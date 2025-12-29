use leptos::logging::error;
use leptos::prelude::*;

use ipchat::proto::Room;

use crate::components::molecules::Sidebar;
use crate::components::organisms::ChatArea;
use crate::hooks::chat_websocket::use_chat_ws;
use crate::hooks::node::use_server_url;

#[component]
pub fn Chat() -> impl IntoView {
    let username = RwSignal::new(String::from("myuser"));
    let is_sidebar_open = RwSignal::new(true);
    let is_connected = RwSignal::new(false);
    let my_rooms = RwSignal::new(vec![]);
    let discovered_peers = RwSignal::new(vec![]);
    let active_room: RwSignal<Option<Room>> = RwSignal::new(None);
    let set_active_room = {
        let active_room = active_room.clone();
        move |room: Room| {
            let active_room = active_room.clone();
            active_room.set(Some(room.clone()));
        }
    };
    let open_sidebar = {
        let is_sidebar_open = is_sidebar_open.clone();
        move || {
            is_sidebar_open.set(true);
        }
    };
    let close_sidebar = {
        let is_sidebar_open = is_sidebar_open.clone();
        move || {
            is_sidebar_open.set(false);
        }
    };
    let request_peer_list = || {
        // Implementation to request peer list
    };
    let create_room = |room_name: String| {
        // Implementation to create a new room
    };

    Effect::new(move || {
        let Some(server_url) = use_server_url().get() else {
            return;
        };

        if let Err(err) = use_chat_ws().connect(server_url) {
            error!("Failed to connect to chat WebSocket: {:?}", err);
        }
    });

    view! {
        <div>
            <Sidebar
                username={username.read_only()}
                is_sidebar_open={is_sidebar_open.read_only()}
                open_sidebar={open_sidebar}
                close_sidebar={close_sidebar}
                is_connected={is_connected.read_only()}
                set_active_room={set_active_room}
                my_rooms={my_rooms.read_only()}
                discovered_peers={discovered_peers.read_only()}
                active_room={active_room.read_only()}
                request_peer_list={request_peer_list}
                create_room={create_room}
            />
            <ChatArea
                username={username.read_only()}
                active_room={active_room.read_only()}
                send_message={|room_id, username, content| {
                    // Implementation to send message
                }}
            />
        </div>
    }
}
