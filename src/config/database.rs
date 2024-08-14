use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    user: String,
    password: secrecy::Secret<String>,
    name: String,
    require_ssl: bool,
}

impl Settings {
    pub fn get_db_pool(&self) -> sqlx::PgPool {
        use secrecy::ExposeSecret;

        let ssl_mode = if self.require_ssl {
            sqlx::postgres::PgSslMode::Require
        } else {
            sqlx::postgres::PgSslMode::Prefer
        };

        let options = sqlx::postgres::PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.user)
            .password(&self.password.expose_secret())
            .database(&self.name)
            .ssl_mode(ssl_mode);

        sqlx::PgPool::connect_lazy_with(options)
    }
}
