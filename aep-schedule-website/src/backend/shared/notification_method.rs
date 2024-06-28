use crate::backend::shared::email::Email;

pub enum NotificationMethod {
    Email(Email),
    //PushNotification(PushNotification),
}

impl NotificationMethod {
    pub fn send_notification(&self) {}
}
