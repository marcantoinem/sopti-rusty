use std::env;
use super::{auth_token::AuthToken, users::SigleGroup};
use lettre::{message::{header::ContentType, Mailbox}, transport::smtp::authentication::Credentials, Message, SmtpTransport, AsyncTransport};

pub struct Email {
    email: String,
    auth_token: AuthToken,
}

impl Email {
    fn template_header(&self, sigle_group: SigleGroup) -> String {
        format!(
            "La section de {} {} du cours {} s'est ouverte.",
            sigle_group.sigle,
            sigle_group.group_index.to_usize() + 1,
            sigle_group.group_type.to_string()
        )
    }

    fn template_body(&self, sigle_group: SigleGroup) -> String {
        format!(
            "La section de {} {} du cours {} s'est ouverte.",
            sigle_group.sigle,
            sigle_group.group_index.to_usize() + 1,
            sigle_group.group_type.to_string()
        )
    }

    pub fn authentify_smtp() -> SmtpTransport {
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

    pub async fn send_message(&self, sigle_group: SigleGroup, mailer: &SmtpTransport) {
        let email = Message::builder()
            .from("Marc-Antoine Manningham <marc-antoine.manningham@polymtl.ca>".parse().unwrap())
            .to(Mailbox::new(None, self.email.parse().unwrap()))
            .subject(self.template_header(sigle_group))
            .header(ContentType::TEXT_PLAIN)
            .body(self.template_body(sigle_group))
            .unwrap();
        
        match mailer.send(&email).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}