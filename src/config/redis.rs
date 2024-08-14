#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    url: String,
}

impl Settings {
    pub async fn get_redis_async_conn(&self) -> redis::aio::MultiplexedConnection {
        let client =
            redis::Client::open(self.url.as_str()).expect("failed to open a redis client.");

        client
            .get_multiplexed_async_connection()
            .await
            .expect("failed to establish a multiplexed async connection with redis")
    }
}
