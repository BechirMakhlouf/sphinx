use std::sync::Arc;

use sqlx::PgPool;

use crate::models::{
    email::Email,
    identity::Provider,
    password::EncryptedPassword,
    user::{self, AuthenticatedData},
};

pub struct AuthRepository {
    db_pool: PgPool,
}
impl AuthRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
    pub async fn get_authenticated_user_data(
        &self,
        email: Email,
        encrypted_password: EncryptedPassword,
    ) -> super::Result<AuthenticatedData> {
        Ok(sqlx::query_as!(
            AuthenticatedData,
            r#"
            SELECT
                user_id AS "user_id: user::Id", 
                auth.users.email AS "email: Email", 
                auth.users.phone,
                provider AS "provider: Provider",
                provider_data,
                is_admin
            FROM auth.users
            JOIN auth.identities ON auth.users.id = auth.identities.user_id
            WHERE auth.identities.provider = $1 AND encrypted_password = $2 AND auth.users.email = $3;
        "#,
            Provider::Email as Provider,
            encrypted_password as EncryptedPassword,
            email as Email,
        )
        .fetch_one(&self.db_pool)
        .await?)
    }
}
