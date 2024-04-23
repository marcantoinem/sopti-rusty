use self::{email::Email, push_notification::PushNotification, users::SigleGroup};

pub mod users;
pub mod auth_token;
pub mod push_notification;
pub mod email;


pub(super) trait SendNotification {
    fn send_message(&self, sigle_group: SigleGroup);
}

pub enum NotificationMethod {
    Email(Email),
    PushNotification(PushNotification),
}