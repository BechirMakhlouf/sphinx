use secrecy::ExposeSecret;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Password(secrecy::Secret<String>);

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct EncryptedPassword(String);

//TODO: Make suitable errors

impl TryFrom<&str> for Password {
    type Error = ();
    fn try_from(password: &str) -> Result<Self, Self::Error> {
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
    type Error = ();
    fn try_from(encrypted_password: &str) -> Result<Self, Self::Error> {
        use argon2::PasswordVerifier;

        let parsed_hash = argon2::PasswordHash::new(&encrypted_password).unwrap();

        match argon2::Argon2::default().verify_password(encrypted_password.as_bytes(), &parsed_hash)
        {
            Ok(_) => Ok(Self(encrypted_password.to_string())),
            Err(_) => Err(()),
        }
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
