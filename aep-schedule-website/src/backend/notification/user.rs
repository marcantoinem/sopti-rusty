use super::auth_token::AuthToken;
use aep_schedule_generator::data::group_sigle::SigleGroup;

pub struct User {
    notifications_method: Vec<NotificationMethod>,
    auth_token: AuthToken,
    sigles_group: Vec<SigleGroup>,
}

#[derive(Clone, Debug)]
pub struct SharedUser(Arc<Mutex<User>>);

impl SharedUser {
    pub fn new() -> Self {}
}
