use self::{email::Email, push_notification::PushNotification, users::SigleGroup};

pub mod users;
pub mod auth_token;
pub mod push_notification;
pub mod email;

pub enum NotificationMethod {
    Email(Email),
    PushNotification(PushNotification),
}