use anyhow::Result;

use crate::domain::user::model::User;
use crate::domain::user::repository::UserRepository;
use crate::setup::Setup;

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(setup: Setup) -> Self {
        let repository = UserRepository::new(setup);
        UserService { repository }
    }

    pub async fn create_user(&self, username: &str) -> Result<User> {
        self.repository.insert(username).await
    }
}
