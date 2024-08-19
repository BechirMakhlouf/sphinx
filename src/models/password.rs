use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("Provided password is weak: {0}")]
    WeakPassword(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Password(secrecy::Secret<String>);

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct EncryptedPassword(String);

impl TryFrom<&str> for Password {
    type Error = Error;
    fn try_from(password: &str) -> Result<Self> {
        Ok(Self(secrecy::Secret::new(password.into())))
    }
}

impl Password {
    pub fn encrypt(&self) -> EncryptedPassword {
        use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};

        let salt = SaltString::generate(&mut OsRng);

        EncryptedPassword::from_trusted_string(
            argon2::Argon2::default()
                .hash_password(self.0.expose_secret().as_bytes(), &salt)
                .unwrap()
                .to_string(),
        )
    }
}

impl TryFrom<&str> for EncryptedPassword {
    type Error = Error;
    fn try_from(encrypted_password: &str) -> Result<Self> {
        use argon2::PasswordVerifier;

        let parsed_hash = argon2::PasswordHash::new(&encrypted_password).unwrap();

        match argon2::Argon2::default().verify_password(encrypted_password.as_bytes(), &parsed_hash)
        {
            Ok(_) => Ok(Self(encrypted_password.to_string())),
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }
}
impl ToString for EncryptedPassword {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl EncryptedPassword {
    pub fn from_trusted_str(encrypted_password: &str) -> Self {
        Self(encrypted_password.to_string())
    }

    pub fn from_trusted_string(encrypted_password: String) -> Self {
        Self(encrypted_password)
    }

    pub fn compare_with(&self, password: &Password) -> bool {
        use argon2::PasswordVerifier;

        let parsed_hash = argon2::PasswordHash::new(&self.0).unwrap();

        argon2::Argon2::default()
            .verify_password(password.0.expose_secret().as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
