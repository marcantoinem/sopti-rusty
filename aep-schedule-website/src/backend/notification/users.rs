use crate::backend::shared::user_builder::UserBuilder;

use super::{auth_token::AuthToken, user::SharedUser};
use aep_schedule_generator::data::group_sigle::SigleGroup;
use compact_str::CompactString;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use std::{collections::HashMap, env, fmt::Display, sync::Arc};
use tower::make::Shared;

struct UsersToNotify {
    courses: HashMap<SigleGroup, Vec<SharedUser>>,
    users: HashMap<AuthToken, SharedUser>,
    create_mailer: SmtpTransport,
}

impl UsersToNotify {
    fn create_mailer() -> SmtpTransport {
        let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME env variable not defined");
        let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable not defined");
        let creds = Credentials::new(username, password);

        let relay = env::var("SMTP_RELAY").expect("SMTP_RELAY_URL env variable not defined");
        let port: u16 = env::var("SMTP_PORT")
            .expect("SMTP_PORT env variable not defined")
            .parse()
            .expect("SMTP_PORT env variable should be a number");

        SmtpTransport::starttls_relay(&relay)
            .unwrap()
            .credentials(creds)
            .port(port)
            .build()
    }

    pub fn new<'a>(courses: impl Iterator<Item = &'a SigleGroup>) -> Self {
        let create_mailer = Self::create_mailer();
        let courses = courses.map(|c| (c.clone(), vec![])).collect();
        let users = HashMap::new();

        Self {
            courses,
            users,
            create_mailer,
        }
    }

    pub fn register_user(&mut self, user: UserBuilder) {
        let user: SharedUser = user.into();
        user.for_each_groups(|g| {
            self.courses
                .entry(g.clone())
                .and_modify(|v| v.push(user.clone()))
                .or_insert(vec![user.clone()]);
        });
    }
}
