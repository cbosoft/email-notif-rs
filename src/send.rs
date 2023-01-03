use std::panic::{catch_unwind, resume_unwind, UnwindSafe};

use crate::config::Config;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

/// Struct containing config and a tag. Associated methods are used to send
/// email and to enclose functions in status updates.
pub struct EmailNotifier {
    config: Config,
    tag: String,
}

impl EmailNotifier {
    /// Constructs a new `EmailNotifier`. The parameter `tag` is a descriptive
    /// name given to the process that you are monitoring. This tag will be
    /// included in the subject line of any sent emails. Configuration is
    /// loaded from the default location.
    ///
    /// # Example
    /// ```
    /// use email_notif::EmailNotifier;
    /// let em = EmailNotifier::new();
    /// ```
    pub fn new(tag: impl ToString) -> Self {
        EmailNotifier {
            config: Config::load(),
            tag: tag.to_string(),
        }
    }

    /// Method to send an email via SMTP with the given subject and plain-text
    /// body.
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

    /// Send an update email about the running process, with the given body text.
    pub fn send_update(&self, body: String) {
        self.send_email(format!("{} Update", self.tag), body);
    }

    /// Send a message indicating the process has completed successfully.
    pub fn send_success(&self) {
        self.send_email(
            format!("{} Complete", self.tag),
            format!("{} has completed successfully.", self.tag),
        );
    }

    /// Send a message indicating the process has resulted in a panic.
    pub fn send_error(&self) {
        self.send_email(
            format!("{} Error!", self.tag),
            format!("{} has encountered a error and has panicked.", self.tag),
        );
    }

    /// Run a closure and send an email when the closure completes
    /// successfully (`EmailNotifier::send_success`) or if the process
    /// results in a panic, send an error message
    /// (`EmailNotifier::send_error`)
    ///
    /// # Example
    ///
    /// ```
    /// use email_notif::EmailNotifier;
    /// EmailNotifier::new("Test").capture(|em|{
    ///    for i in 0..10 {
    ///      em.send_update(format!("iteration {i} complete."));
    ///    }
    /// });
    /// ```
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
