use super::notification_method::NotificationMethod;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use std::collections::HashSet;

pub struct UserBuilder {
    pub notifications_method: Vec<NotificationMethod>,
    pub sigles_group: HashSet<SigleGroup>,
}
