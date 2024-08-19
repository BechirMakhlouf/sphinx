use redis::{AsyncCommands, RedisError};

use crate::models::user;

pub struct TokenRepository {
    redis_client: redis::Client,
    token_expiry_secs: u64,
}

impl TokenRepository {
    pub fn new(redis_client: redis::Client, token_expiry_secs: u64) -> Self {
        Self {
            redis_client,
            token_expiry_secs,
        }
    }
    pub async fn store_refresh_token(
        &self,
        user_id: &user::Id,
        token_id: String,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!("refresh_token:{}:{}", user_id.to_string(), token_id);

        conn.set_ex(key, "", self.token_expiry_secs).await
    }

    pub async fn refresh_token_exists(
        &self,
        user_id: user::Id,
        token_id: String,
    ) -> Result<bool, RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!(
            "refresh_token:{}:{}",
            user_id.as_uuid().to_string(),
            token_id
        );

        conn.exists(key).await
    }

    pub async fn delete_refresh_token(
        &self,
        user_id: user::Id,
        token_id: String,
    ) -> Result<bool, RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!("refresh_token:{}:{}", user_id.to_string(), token_id);

        conn.del::<String, bool>(key).await
    }
}
