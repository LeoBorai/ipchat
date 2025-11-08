use crate::{domain::user::service::UserService, setup::Setup};

pub mod user;

pub struct Services {
    pub user: UserService,
}

impl Services {
    pub fn new(setup: Setup) -> Self {
        Self {
            user: UserService::new(setup.clone()),
        }
    }
}
