use self::{email::Email, push_notification::PushNotification, users::SigleGroup};

pub mod auth_token;
pub mod email;
pub mod push_notification;
pub mod user;
pub mod users;

pub enum NotificationMethod {
    Email(Email),
    PushNotification(PushNotification),
}

impl NotificationMethod {
    pub fn send_notification(&self) {}
}
