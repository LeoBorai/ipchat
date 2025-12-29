use leptos::logging::error;
use leptos::prelude::{RwSignal, Set};

use ipchat_client::{ApiClient, NodeObject};

#[derive(Clone, Debug)]
pub struct NodeContext {
    pub node_info: RwSignal<Option<NodeObject>>,
    pub server_url: RwSignal<String>,
    pub is_loading: RwSignal<bool>,
}

impl Default for NodeContext {
    fn default() -> Self {
        let ctx = Self {
            node_info: RwSignal::new(None),
            server_url: RwSignal::new("ws://localhost:8080".to_string()),
            is_loading: RwSignal::new(true),
        };

        leptos::task::spawn_local({
            let ctx = ctx.clone();
            async move {
                ctx.fetch_node_info().await;
            }
        });

        ctx
    }
}

impl NodeContext {
    async fn fetch_node_info(&self) {
        let ipchat = ApiClient::remote("http://localhost:4724");

        match ipchat.node_info().await {
            Ok(node_object) => {
                let web_socket_port = node_object
                    .web_socket_addr
                    .split(':')
                    .nth(1)
                    .unwrap_or("8080");
                let new_server_url = if node_object.client_ip.starts_with("127.0.0.1") {
                    format!("ws://localhost:{}", web_socket_port)
                } else {
                    let server_local_ip_addr = node_object
                        .local_ip
                        .split(':')
                        .next()
                        .unwrap_or("localhost");
                    format!("ws://{}:{}", server_local_ip_addr, web_socket_port)
                };

                self.server_url.set(new_server_url);
                self.node_info.set(Some(node_object));
            }
            Err(err) => {
                error!("Failed to fetch node info: {:?}", err);
            }
        }

        self.is_loading.set(false);
    }
}
