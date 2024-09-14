use chrono::Utc;
use sqlx::{postgres::PgQueryResult, PgPool};

use crate::models::user;

#[derive(Debug, Clone)]
pub struct AuthRepository {
    db_pool: PgPool,
}

impl AuthRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
    pub async fn confirm_user_email(&self, user_id: &user::Id) -> super::Result<PgQueryResult> {
        sqlx::query!(
            "UPDATE auth.users 
            SET 
                email_confirmed_at = $1
            WHERE
                id = $2;",
            Utc::now(),
            user_id as &user::Id
        )
        .execute(&self.db_pool)
        .await
    }
}
