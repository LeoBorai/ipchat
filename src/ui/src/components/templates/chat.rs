use leptos::logging::error;
use leptos::prelude::*;

use crate::components::molecules::Sidebar;
use crate::components::organisms::ChatArea;
use crate::hooks::chat_websocket::use_chat_ws;
use crate::hooks::node::use_server_url;

#[component]
pub fn Chat() -> impl IntoView {
    let username = RwSignal::new(String::from("myuser"));

    Effect::new(move || {
        let Some(server_url) = use_server_url().get() else {
            return;
        };

        if let Err(err) = use_chat_ws().connect(server_url) {
            error!("Failed to connect to chat WebSocket: {:?}", err);
        }
    });

    view! {
        <div class="flex h-screen bg-gray-100">
            <Sidebar username=username.read_only() />
            <ChatArea username=username.read_only() />
        </div>
    }
}
