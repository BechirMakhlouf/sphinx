#![allow(unused)]
use secrecy::ExposeSecret;
use serde_aux::field_attributes::deserialize_number_from_string;

static CONFIG_FILE_NAME: &str = "config.yaml";

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    user: String,
    password: secrecy::Secret<String>,
    name: String,
    require_ssl: bool,
}

impl DatabaseSettings {
    pub fn get_db_pool(&self) -> sqlx::PgPool {
        let ssl_mode = if self.require_ssl {
            sqlx::postgres::PgSslMode::Require
        } else {
            sqlx::postgres::PgSslMode::Prefer
        };

        let options = sqlx::postgres::PgConnectOptions::new()
            .host(&self.host)
            .username(&self.user)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode);

        sqlx::PgPool::connect_lazy_with(options)
    }
}

pub fn get_config() -> Result<Config, ()> {
    let config_path = std::env::current_dir()
        .expect("couldn't get current working directory")
        .join(CONFIG_FILE_NAME);

    let config = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .expect("error while building config");

    Ok(config
        .try_deserialize::<Config>()
        .expect("error while fetching config files"))
}
