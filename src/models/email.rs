#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error)]
pub enum Error {
    #[error("Provided email is invalid: {0}")]
    InvalidEmail(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct Email(String);

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Email {
    type Error = Error;

    fn try_from(email: &str) -> Result<Self> {
        let is_valid = validator::ValidateEmail::validate_email(&email);

        match is_valid {
            true => Ok(Self(email.into())),
            false => Err(Error::InvalidEmail(email.into())),
        }
    }
}

impl Email {
    pub fn from_trusted_str(email: &str) -> Self {
        Self(email.to_string())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn to_mailbox(&self) -> lettre::message::Mailbox {
        let name = &self.0.split_once("@").unwrap().0.to_string();
        lettre::message::Mailbox::new(Some(name.to_string()), self.0.parse().unwrap())
    }
}
