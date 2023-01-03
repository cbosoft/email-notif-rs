use std::panic::{catch_unwind, resume_unwind, UnwindSafe};

use crate::config::Config;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub struct EmailNotifier {
    config: Config,
    tag: String,
}

impl EmailNotifier {
    pub fn new(tag: impl ToString) -> Self {
        EmailNotifier {
            config: Config::load(),
            tag: tag.to_string(),
        }
    }

    fn send_email(&self, subject: String, body: String) {
        let email = Message::builder()
            .from(self.config.sender_email.parse().unwrap())
            .to(self.config.recipient_email.parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();

        let creds = Credentials::new(
            self.config.sender_email.clone(),
            self.config.password.clone(),
        );
        let mailer = SmtpTransport::relay(&self.config.smtp_server)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => (),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }

    pub fn send_update(&self, body: String) {
        self.send_email(format!("{} Update", self.tag), body);
    }

    fn send_success(&self) {
        self.send_email(
            format!("{} Complete", self.tag),
            format!("{} has completed successfully.", self.tag),
        );
    }

    fn send_error(&self) {
        self.send_email(
            format!("{} Error!", self.tag),
            format!("{} has encountered a error and has panicked.", self.tag),
        );
    }

    pub fn capture<F>(self, f: F)
    where
        F: UnwindSafe + FnOnce(&EmailNotifier) -> (),
    {
        match catch_unwind(|| {
            f(&self);
        }) {
            Ok(_) => self.send_success(),
            Err(e) => {
                self.send_error();
                resume_unwind(e);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_mail() {
        let em = EmailNotifier::new("Test1");
        em.send_email("Test".to_string(), "<b>test</b>".to_string());
    }

    #[test]
    fn test_capture() {
        EmailNotifier::new("Test2").capture(|_| {});
    }

    #[test]
    #[should_panic]
    fn test_capture_error() {
        EmailNotifier::new("Test3").capture(|_| panic!("foo"));
    }
}
