use super::auth_token::AuthToken;
use crate::backend::shared::email::Email;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncTransport, Message, SmtpTransport, Transport,
};
use std::env;

impl Email {
    fn template_header(&self, sigle_group: &SigleGroup) -> String {
        format!(
            "La section de {} {} du cours {} s'est ouverte.",
            sigle_group.sigle,
            sigle_group.group_index.to_usize() + 1,
            sigle_group.group_type.to_string()
        )
    }

    fn template_body(&self, sigle_group: &SigleGroup) -> String {
        format!(
            "La section de {} {} du cours {} s'est ouverte.",
            sigle_group.sigle,
            sigle_group.group_index.to_usize() + 1,
            sigle_group.group_type.to_string()
        )
    }

    pub async fn send_message(&self, sigle_group: SigleGroup, mailer: &SmtpTransport) {
        let email = Message::builder()
            .from(
                "Marc-Antoine Manningham <marc-antoine.manningham@polymtl.ca>"
                    .parse()
                    .unwrap(),
            )
            .to(Mailbox::new(None, self.email.parse().unwrap()))
            .subject(self.template_header(&sigle_group))
            .header(ContentType::TEXT_PLAIN)
            .body(self.template_body(&sigle_group))
            .unwrap();

        match mailer.send(&email) {
            Err(e) => log::error!("Sending mail failed with error: {}", e),
            _ => (),
        };
    }
}
