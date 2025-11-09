use std::sync::Arc;

use crate::domain::user::service::UserService;
use crate::setup::Setup;
use crate::ws::WebSocket;

pub type SharedServices = Arc<Services>;

pub struct Services {
    pub setup: Setup,
    pub user: UserService,
    pub web_socket: Arc<WebSocket>,
}

impl Services {
    pub fn new(setup: Setup, web_socket: Arc<WebSocket>) -> SharedServices {
        Arc::new(Self {
            setup: setup.clone(),
            user: UserService::new(setup.clone()),
            web_socket,
        })
    }
}
