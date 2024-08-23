#![allow(unused)]
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub iss: String,
    pub aud: Vec<String>,
    pub access_token: AccessTokenSettings,
    pub refresh_token: RefreshTokenSettings,
    pub default_token: DefaultTokenSettings,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AccessTokenSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub exp_duration_secs: u64,
    pub secret: secrecy::Secret<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RefreshTokenSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub exp_duration_secs: u64,
    pub secret: secrecy::Secret<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DefaultTokenSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub exp_duration_secs: u64,
    pub secret: secrecy::Secret<String>,
}
