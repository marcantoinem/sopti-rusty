use aep_schedule_generator::data::group_sigle::SigleGroup;

use super::notification_method::NotificationMethod;

pub struct UserBuilder {
    pub notifications_method: Vec<NotificationMethod>,
    pub sigles_group: Vec<SigleGroup>,
}
