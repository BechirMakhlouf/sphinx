use sqlx::{types::ipnetwork::IpNetwork, PgPool};

use super::Result;
use crate::models::{
    session::{self, Session},
    user,
};

#[derive(Debug, Clone)]
pub struct SessionRepository {
    db_pool: PgPool,
}

impl SessionRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
    pub async fn add_session(&self, session: &Session) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO auth.sessions ( id, user_id, user_agent, ip) VALUES ($1, $2, $3, $4)"#,
            &session.id as &session::Id,
            &session.user_id as &user::Id,
            session.user_agent,
            IpNetwork::from(session.ip)
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
