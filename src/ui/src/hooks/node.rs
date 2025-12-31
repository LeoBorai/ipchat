use leptos::prelude::{Get, Signal, expect_context};

use crate::contexts::node_context::NodeContext;

pub fn use_node() -> NodeContext {
    expect_context::<NodeContext>()
}

pub fn use_server_url() -> Signal<Option<String>> {
    Signal::derive(move || use_node().server_url.get())
}
