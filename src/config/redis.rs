#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    url: url::Url,
}

impl Settings {
    pub async fn get_async_connection(&self) -> redis::aio::MultiplexedConnection {
        let client =
            redis::Client::open(self.url.as_str()).expect("failed to open a redis client.");

        client
            .get_multiplexed_async_connection()
            .await
            .expect("failed to establish a multiplexed async connection with redis")
    }

    pub fn get_client(&self) -> redis::Client {
        redis::Client::open(self.url.as_str()).expect("failed to open a redis client.")
    }
}
