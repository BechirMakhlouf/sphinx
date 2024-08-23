use lettre::{message::Mailbox, AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

use crate::{config::smtp, models::email::Email};

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    #[error("Internal Error: {0}.")]
    InternalError(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
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

    /// send confirmation email.
    pub async fn send_confirm_email(&self, recipient: &Email, link: &url::Url) -> Result<bool> {
        let email = lettre::Message::builder()
            .from(self.mailbox.clone())
            .to(recipient.to_mailbox())
            .subject("Confirm Email")
            .header(lettre::message::header::ContentType::TEXT_PLAIN)
            .body(String::from(format!("Confirm your Email: {}", link)))
            .unwrap();

        match self.smtp_transport.send(email).await {
            Ok(v) => Ok(v.is_positive()),
            Err(e) => Err(Error::InternalError(e.to_string())),
        }
    }
    /// sends reset_password email.
    pub async fn send_reset_password(&self, recipient: &Email, link: &url::Url) -> Result<bool> {
        let email = lettre::Message::builder()
            .from(self.mailbox.clone())
            .to(recipient.to_mailbox())
            .subject("Reset Password")
            .header(lettre::message::header::ContentType::TEXT_PLAIN)
            .body(String::from(format!("Reset your Password: {}", link)))
            .unwrap();

        match self.smtp_transport.send(email).await {
            Ok(v) => Ok(v.is_positive()),
            Err(e) => Err(Error::InternalError(e.to_string())),
        }
    }
    // send otp password.
    // send magic link
}
