use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Address, AsyncSmtpTransport,
    Tokio1Executor,
};
use secrecy::ExposeSecret;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    url: String,
    username: String,
    password: secrecy::Secret<String>,
    mailbox: MailboxSettings,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MailboxSettings {
    email: String,
    name: String,
}

impl Settings {
    pub fn get_smtp_transport(&self) -> AsyncSmtpTransport<Tokio1Executor> {
        let creds = Credentials::new(
            self.username.clone(),
            self.password.expose_secret().to_string(),
        );

        AsyncSmtpTransport::<Tokio1Executor>::relay(self.url.as_str())
            .expect("Invalid smtp server url in config.")
            .credentials(creds)
            .build()
    }
    pub fn get_mailbox(&self) -> Mailbox {
        Mailbox::new(
            Some(self.mailbox.name.clone()),
            self.mailbox
                .email
                .parse::<Address>()
                .expect("Invalid email address in config."),
        )
    }
}
