use leptos::prelude::{Get, Signal, expect_context};

use crate::contexts::ui_context::UIContext;

pub fn use_ui() -> UIContext {
    expect_context::<UIContext>()
}

pub fn use_is_sidebar_open() -> Signal<bool> {
    Signal::derive(move || use_ui().is_sidebar_open.get())
}

pub fn use_toggle_sidebar() -> impl Fn() {
    let ui = use_ui();
    move || {
        ui.toggle_sidebar();
    }
}
