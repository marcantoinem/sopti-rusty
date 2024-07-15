use super::auth_token::AuthToken;
use crate::backend::shared::email::Email;
use crate::backend::shared::user_builder::UserBuilder;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use lettre::SmtpTransport;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

struct User {
    pub email: Email,
    //notification: Option<Notification>,
    pub auth_token: AuthToken,
    pub sigles_group: HashSet<SigleGroup>,
}

#[derive(Clone)]
pub struct SharedUser(Arc<Mutex<User>>);

impl SharedUser {
    pub fn add_group(&mut self, sigle_group: SigleGroup) {
        self.0.lock().unwrap().sigles_group.insert(sigle_group);
    }
    pub fn for_each_groups(&self, mut to_apply: impl FnMut(&SigleGroup) -> ()) {
        for g in &self.0.lock().unwrap().sigles_group {
            to_apply(g);
        }
    }
    pub fn get_auth_token(&self) -> AuthToken {
        self.0.lock().unwrap().auth_token.clone()
    }
    pub async fn send_message(&self, sigle_group: &HashSet<SigleGroup>, mailer: &SmtpTransport) {
        self.0
            .lock()
            .unwrap()
            .email
            .send_message(sigle_group, mailer)
            .await;
    }
}

impl From<UserBuilder> for SharedUser {
    fn from(value: UserBuilder) -> Self {
        let UserBuilder { email, sigle_group } = value;
        let mut sigles_group = HashSet::new();
        sigles_group.insert(sigle_group);
        let user = User {
            email,
            auth_token: AuthToken::new(),
            sigles_group,
        };
        Self(Arc::new(Mutex::new(user)))
    }
}
