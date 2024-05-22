use std::{collections::HashMap, fmt::Display, sync::Arc};
use compact_str::CompactString;
use lettre::SmtpTransport;
use crate::data::group_index::GroupIndex;
use super::{auth_token::AuthToken, NotificationMethod};

#[derive(Debug, Clone, Copy, Hash)]
#[repr(u8)]
pub enum GroupType {
    LabGroup,
    TheoGroup,
}

impl Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LabGroup => f.write_str("laboratoire"),
            Self::TheoGroup => f.write_str("théorie"),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SigleGroup {
    pub sigle: CompactString,
    pub group_type: GroupType,
    pub group_index: GroupIndex,
}

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