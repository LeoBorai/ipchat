use leptos::prelude::{Get, Signal, expect_context};

use ipchat_client::NodeObject;

use crate::contexts::node_context::NodeContext;

pub fn use_node() -> NodeContext {
    expect_context::<NodeContext>()
}

pub fn use_server_url() -> Signal<String> {
    Signal::derive(move || use_node().server_url.get())
}

pub fn use_is_loading() -> Signal<bool> {
    Signal::derive(move || use_node().is_loading.get())
}
