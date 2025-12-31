use leptos::prelude::{Get, Signal, expect_context};

use crate::contexts::session_context::SessionContext;

pub fn use_session() -> SessionContext {
    expect_context::<SessionContext>()
}

pub fn use_username() -> Signal<Option<String>> {
    Signal::derive(move || use_session().username.get())
}
