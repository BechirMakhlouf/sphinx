use redis::{AsyncCommands, RedisError};

use crate::models::{session, token::REFRESH_TOKEN_NAME, user};

#[derive(Debug, Clone)]
pub struct TokenRepository {
    redis_client: redis::Client,
    refresh_token_expiry_secs: u64,
}

//TODO: fix opening multiple async connections

impl TokenRepository {
    pub fn new(redis_client: redis::Client, refresh_token_expiry_secs: u64) -> Self {
        Self {
            redis_client,
            refresh_token_expiry_secs,
        }
    }

    pub async fn store_reset_password_token(
        &self,
        user_id: &user::Id,
        token_id: &str,
        expiry_secs: u64,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!("reset_password:{}:{}", user_id.to_string(), token_id);

        conn.set_ex(key, "", expiry_secs).await
    }

    pub async fn remove_reset_password_token(
        &self,
        user_id: &user::Id,
        token_id: &str,
    ) -> Result<Option<String>, redis::RedisError> {
        let key = format!("reset_password:{}:{}", user_id.to_string(), token_id);
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        conn.get_del(key).await
    }

    pub async fn store_refresh_token(
        &self,
        user_id: &user::Id,
        token_id: &uuid::Uuid,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!(
            "{}:{}:{}",
            REFRESH_TOKEN_NAME,
            user_id.to_string(),
            token_id
        );

        conn.set_ex(key, "", self.refresh_token_expiry_secs).await
    }

    pub async fn refresh_token_exists(
        &self,
        user_id: &user::Id,
        session_id: &str,
    ) -> Result<bool, RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!(
            "{}:{}:{}",
            REFRESH_TOKEN_NAME,
            user_id.to_string(),
            session_id,
        );

        conn.exists(key).await
    }

    pub async fn delete_refresh_token(
        &self,
        user_id: &user::Id,
        session_id: &session::Id,
    ) -> Result<bool, RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = format!(
            "{}:{}:{}",
            REFRESH_TOKEN_NAME,
            user_id.to_string(),
            session_id.to_string()
        );

        conn.del::<String, bool>(key).await
    }
}
