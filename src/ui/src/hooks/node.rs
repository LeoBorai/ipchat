use leptos::prelude::{Signal, expect_context};

use crate::contexts::node_context::{NodeContext, NodeInfo};

pub fn use_node() -> NodeContext {
    expect_context::<NodeContext>()
}

// Read-only accessor functions (optional, for more ergonomic access)
pub fn use_node_info() -> Signal<Option<NodeInfo>> {
    use_node().node_info.read_only()
}

pub fn use_server_url() -> Signal<String> {
    use_node().server_url.read_only()
}

pub fn use_is_loading() -> Signal<bool> {
    use_node().is_loading.read_only()
}
