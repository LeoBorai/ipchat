use anyhow::Result;

use crate::domain::user::model::User;
use crate::setup::Setup;

pub struct UserRepository {
    setup: Setup,
}

impl UserRepository {
    pub fn new(setup: Setup) -> Self {
        Self { setup }
    }

    pub async fn insert(&self, _username: &str) -> Result<User> {
        // Implementation for creating a user in the database
        unimplemented!()
    }
}
