use crate::backend::shared::user_builder::UserBuilder;

use super::{auth_token::AuthToken, user::SharedUser};
use aep_schedule_generator::data::group_sigle::SigleGroup;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use std::{
    collections::{HashMap, HashSet},
    env,
};

pub struct UsersToNotify {
    courses: HashMap<SigleGroup, Vec<SharedUser>>,
    _users: HashMap<AuthToken, SharedUser>,
    mailer: SmtpTransport,
}

impl UsersToNotify {
    fn create_mailer() -> SmtpTransport {
        let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME env variable not defined");
        let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable not defined");
        let creds = Credentials::new(username, password);

        let relay = env::var("SMTP_RELAY").expect("SMTP_RELAY env variable not defined");
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

    pub fn new() -> Self {
        let mailer = Self::create_mailer();
        let courses = HashMap::new();
        let _users = HashMap::new();

        Self {
            courses,
            _users,
            mailer,
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

    pub async fn send_opened(&self, opened: HashSet<SigleGroup>) {
        let mut notified_users = HashSet::new();
        for sigle_group in &opened {
            println!(
                "Group {} of course {} is opened",
                sigle_group.group_index, sigle_group.sigle
            );
            if let Some(users) = self.courses.get(sigle_group) {
                for user in users
                    .iter()
                    .filter(|u| !notified_users.contains(&u.get_auth_token()))
                {
                    user.send_message(&opened, &self.mailer).await;
                }
                notified_users.extend(users.iter().map(|u| u.get_auth_token()));
            }
        }
    }
}
