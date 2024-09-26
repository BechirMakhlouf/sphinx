use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    password_digits_number: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    exp_duration_secs: u32,
}
