mod application;
mod database;
mod jwt;
mod redis;

static CONFIG_FILE_NAME: &str = "config.yaml";

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub application: application::Settings,
    pub database: database::Settings,
    pub redis: redis::Settings,
    pub jwt: jwt::Settings,
}

pub fn get_config() -> Config {
    let config_path = std::env::current_dir()
        .expect("couldn't get current working directory")
        .join(CONFIG_FILE_NAME);

    let config = config::Config::builder()
        .add_source(config::File::from(config_path))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .expect("error while building config");

    config
        .try_deserialize::<Config>()
        .expect("error while fetching config files")
}
