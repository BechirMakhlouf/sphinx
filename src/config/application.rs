use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}
