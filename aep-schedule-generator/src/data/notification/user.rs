use super::token::AuthToken;

enum NotificationMethod {
    Email {
        email: String,
    },
    PushNotification,
}

pub struct User {
    notification_method: Vec<NotificationMethod>,
    identification_hash: AuthToken,
}