use lettre::{message::Mailbox, AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

use crate::config::smtp;

pub struct Mailer {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    mailbox: Mailbox,
}

impl Mailer {
    pub fn new(smtp_settings: smtp::Settings) -> Self {
        Self {
            smtp_transport: smtp_settings.get_smtp_transport(),
            mailbox: smtp_settings.get_mailbox(),
        }
    }

    // send confirmation email.
    pub async fn send_confirm_email(&self, recipient: Mailbox) {
        let email = lettre::Message::builder()
            .from(self.mailbox.clone())
            .to(recipient)
            .subject("Confirm Email")
            .header(lettre::message::header::ContentType::TEXT_PLAIN)
            .body(String::from("please confirm email"))
            .unwrap();

        self.smtp_transport.send(email).await;
    }
    // send reset_password email.
    // send otp password.
    // send magic link
}
