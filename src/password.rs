#[derive(Debug, Clone, Default)]
pub struct PasswordHandler<'a> {
    argon2: argon2::Argon2<'a>,
}

//TODO: remove the unwraps
impl PasswordHandler<'_> {
    pub fn encrypt_password(&self, password: &str) -> String {
        use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};

        let salt = SaltString::generate(&mut OsRng);

        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }

    pub fn verify_password(&self, password: &str, encrypted_password: &str) -> bool {
        use argon2::PasswordVerifier;

        let parsed_hash = argon2::PasswordHash::new(&encrypted_password).unwrap();

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
