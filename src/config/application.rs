use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub config: Config,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub allow_unverified: bool,
    pub reset_password: ResetPasswordSettings,
    pub confirm_email: VerifyEmail,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ResetPasswordSettings {
    pub callback: url::Url,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub token_expiration_secs: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VerifyEmail {
    pub callback: url::Url,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub token_expiration_secs: u64,
}
