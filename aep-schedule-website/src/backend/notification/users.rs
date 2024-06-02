use super::{auth_token::AuthToken, NotificationMethod};
use crate::data::group_index::GroupIndex;
use compact_str::CompactString;
use lettre::SmtpTransport;
use std::{collections::HashMap, fmt::Display, sync::Arc};

struct UsersToNotify {
    courses: HashMap<SigleGroup, Vec<Arc<NotificationMethod>>>,
    users: HashMap<AuthToken, (Arc<NotificationMethod>, Vec<SigleGroup>)>,
    create_mailer: SmtpTransport,
}

impl UsersToNotify {
    fn create_mailer() -> SmtpTransport {
        let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME env variable not defined");
        let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable not defined");
        let creds = Credentials::new(username, password);

        let relay = env::var("SMTP_RELAY").expect("SMTP_RELAY_URL env variable not defined");

        SmtpTransport::starttls_relay(&relay)
            .unwrap()
            .credentials(creds)
            .port(587)
            .build()
    }

    pub fn new(courses: impl Iterator<&CompactString>) -> Self {
        let create_mailer = Self::create_mailer();
        let courses = courses.map(|c| (c, vec![])).collect();
        let users = HashMap::new();

        Self {
            courses,
            users,
            create_mailer,
        }
    }
}
