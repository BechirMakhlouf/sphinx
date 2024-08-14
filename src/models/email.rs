#[derive(Debug, serde::Deserialize, Clone)]
pub struct Email(String);

impl TryFrom<&str> for Email {
    type Error = String;
    fn try_from(email: &str) -> Result<Self, Self::Error> {
        let is_valid = validator::ValidateEmail::validate_email(&email);
        match is_valid {
            true => Ok(Self(email.into())),
            false => Err("Invalid Email".into()),
        }
    }
}
impl Email {
    pub fn parse(email: &str) -> Result<Self, &str> {
        let is_valid = validator::ValidateEmail::validate_email(&email);
        match is_valid {
            true => Ok(Self(email.into())),
            false => Err("Invalid Email"),
        }
    }
    pub fn from_trusted_str(email: &str) -> Self {
        Self(email.to_string())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
