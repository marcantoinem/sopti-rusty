use std::env;

use super::{auth_token::AuthToken, users::SigleGroup};
use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub struct Email {
    email: String,
    auth_token: AuthToken,
}

impl Email {
    pub fn authentify_smtp() -> SmtpTransport {
    let creds = Credentials::new(env::var("SMTP_USERNAME").expect("SMTP_USERNAME env variable not defined").to_string(), env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable not defined").to_string().to_string());
        
        SmtpTransport::starttls_relay("smtp.polymtl.ca")
            .unwrap()
            .credentials(creds)
            .port(587)
            .build()
    }

    pub fn send_message(&self, sigle_group: SigleGroup, mailer: &SmtpTransport) {
        let email = Message::builder()
            .from("Marc-Antoine Manningham <marc-antoine.manningham@polymtl.ca>".parse().unwrap())
            .to("Marc-Antoine Manningham <marc-antoine.manningham@polymtl.ca>".parse().unwrap())
            .subject("Message automatisé")
            .header(ContentType::TEXT_HTML)
            .body("Ceci est un message".to_string())
            .unwrap();
        
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}