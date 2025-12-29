use leptos::IntoView;
use leptos::prelude::{Children, RwSignal, Set, Signal, expect_context, provide_context};
use leptos::logging::error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeInfo {
    #[serde(rename = "webSocketAddr")]
    pub web_socket_addr: String,
    #[serde(rename = "clientIp")]
    pub client_ip: String,
    #[serde(rename = "localIp")]
    pub local_ip: String,
}

#[derive(Clone, Debug)]
pub struct NodeContext {
    pub node_info: RwSignal<Option<NodeInfo>>,
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

        // Fetch node info on initialization
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
        match node_info().await {
            Ok(response) => {
                if let Some(data) = response.data {
                    // Extract WebSocket port from address
                    let web_socket_port = data.web_socket_addr
                        .split(':')
                        .nth(1)
                        .unwrap_or("8080");

                    // Determine server URL based on client IP
                    let new_server_url = if data.client_ip.starts_with("127.0.0.1") {
                        format!("ws://localhost:{}", web_socket_port)
                    } else {
                        let server_local_ip_addr = data.local_ip
                            .split(':')
                            .next()
                            .unwrap_or("localhost");
                        format!("ws://{}:{}", server_local_ip_addr, web_socket_port)
                    };

                    self.server_url.set(new_server_url);
                    self.node_info.set(Some(data));
                }
            }
            Err(err) => {
                error!("Failed to fetch node info: {:?}", err);
            }
        }

        self.is_loading.set(false);
    }

    pub fn set_server_url(&self, url: String) {
        self.server_url.set(url);
    }
}
