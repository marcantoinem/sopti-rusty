use super::email::Email;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBuilder {
    pub email: Option<Email>,
    pub sigles_group: HashSet<SigleGroup>,
}

impl UserBuilder {
    pub fn new(sigles_group: HashSet<SigleGroup>) -> Self {
        Self {
            email: None,
            sigles_group,
        }
    }
    pub fn add_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(Email::new(email.into()));
        self
    }
}
