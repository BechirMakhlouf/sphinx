pub mod application;
pub mod database;
pub mod jwt;
pub mod oauth;
pub mod otp;
pub mod redis;
pub mod smtp;

static CONFIG_FILE_NAME: &str = "config.yaml";

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub application: application::Settings,
    pub database: database::Settings,
    pub redis: redis::Settings,
    pub jwt: jwt::Settings,
    pub oauth: oauth::Settings,
    pub otp: otp::Settings,
    pub smtp: smtp::Settings,
}

pub fn get_config() -> Config {
    dotenv::dotenv().ok();

    let config_path = std::env::current_dir()
        .expect("couldn't get current working directory")
        .join(CONFIG_FILE_NAME);

    let config = config::Config::builder()
        .add_source(config::File::from(config_path))
        .add_source(
            config::Environment::with_prefix("SPHINX")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .expect("error while building config");

    config
        .try_deserialize::<Config>()
        .expect("error while fetching config files")
}
