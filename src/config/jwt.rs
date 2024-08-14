#![allow(unused)]
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    iss: String,
    aud: String,
    access_token: AccessTokenSettings,
    refresh_token: RefreshTokenSettings,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AccessTokenSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    exp_duration_secs: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RefreshTokenSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    exp_duration_secs: u64,
}
