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

// pub trait MailerTrait {
//     fn send_email(
//         &self,
//         recipient: &Email,
//         subject: &str,
//         body: &str,
//     ) -> impl std::future::Future<Output = ()> + Send;
//     fn send_reset_password(
//         &self,
//         recipient: &Email,
//         link: &url::Url,
//     ) -> impl std::future::Future<Output = Result<bool>> + Send;
//
//     fn send_confirm_email(
//         &self,
//         recipient: &Email,
//         link: &url::Url,
//     ) -> impl std::future::Future<Output = Result<bool>> + Send;
// }

impl Mailer {
    pub fn new(smtp_settings: smtp::Settings) -> Self {
        Self {
            smtp_transport: smtp_settings.get_smtp_transport(),
            mailbox: smtp_settings.get_mailbox(),
        }
    }

    // pub async fn send_email(&self, recipient: &Email, subject: &str, body: &str) {
    //     let email = lettre::Message::builder()
    //         .from(self.mailbox.clone())
    //         .subject(subject)
    //         .header(lettre::message::header::ContentType::TEXT_HTML);
    // }
    /// send confirmation email.
    pub async fn send_confirm_email(&self, recipient: &Email, link: &url::Url) -> Result<bool> {
        let email = lettre::Message::builder()
            .from(self.mailbox.clone())
            .to(recipient.to_mailbox())
            .subject("Confirm Email")
            .header(lettre::message::header::ContentType::TEXT_PLAIN)
            .body(format!("Confirm your Email: {}", link))
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
            .body(format!("Reset your Password: {}", link))
            .unwrap();

        match self.smtp_transport.send(email).await {
            Ok(v) => Ok(v.is_positive()),
            Err(e) => Err(Error::InternalError(e.to_string())),
        }
    }
    // send otp password.
    // send magic link
}
