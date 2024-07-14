use super::email::Email;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBuilder {
    pub email: Email,
    pub sigle_group: SigleGroup,
}

impl UserBuilder {
    pub fn new(email: Email, sigle_group: SigleGroup) -> Self {
        Self { email, sigle_group }
    }
}
