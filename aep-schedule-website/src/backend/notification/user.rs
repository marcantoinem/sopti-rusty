use super::auth_token::AuthToken;
use crate::backend::shared::notification_method::NotificationMethod;
use crate::backend::{shared::user_builder::UserBuilder, user_builder::UserBuilder};
use aep_schedule_generator::data::group_sigle::SigleGroup;
use std::sync::Arc;
use std::sync::Mutex;

struct User {
    notifications_method: Vec<NotificationMethod>,
    auth_token: AuthToken,
    sigles_group: Vec<SigleGroup>,
}

#[derive(Clone)]
pub struct SharedUser(Arc<Mutex<User>>);

impl SharedUser {
    pub fn groups(&self) -> &[SigleGroup] {
        &self.0.sigles_group
    }
}

impl From<UserBuilder> for SharedUser {
    fn from(value: UserBuilder) -> Self {
        let UserBuilder {
            notifications_method,
            sigles_group,
        } = value;
        let user = User {
            notifications_method,
            auth_token: AuthToken::new(),
            sigles_group,
        };
        Self(Arc::new(Mutex::new(user)))
    }
}
