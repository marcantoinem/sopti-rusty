use super::email::Email;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use std::collections::HashSet;

pub struct UserBuilder {
    pub email: Option<Email>,
    pub sigles_group: HashSet<SigleGroup>,
}
