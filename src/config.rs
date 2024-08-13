#![allow(unused)]
use serde_aux::field_attributes::deserialize_number_from_string;

static CONFIG_FILE_NAME: &str = "config.yaml";

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    application: ApplicationSettings,
    database: DatabaseSettings,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    host: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    url: String,
    require_ssl: bool,
}

pub fn get_config() -> Result<Config, ()> {
    let config_path = std::env::current_dir().unwrap().join(CONFIG_FILE_NAME);

    let config = config::Config::builder()
        .add_source(config::File::from(config_path))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .unwrap();

    Ok(config.try_deserialize::<Config>().unwrap())
}
