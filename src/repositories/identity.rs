use sqlx::PgPool;

use crate::models::{
    email::Email,
    identity::{Identity, Provider},
    password::EncryptedPassword,
    user::{self, AuthenticatedData},
};

type Result<T> = std::result::Result<T, sqlx::Error>;

#[derive(Debug, Clone)]
pub struct IdentityRepository {
    db_pool: PgPool,
}

impl IdentityRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
    pub async fn get_user_identities(&self, user_id: user::Id) -> Result<Vec<Identity>> {
        Ok(sqlx::query_as!(
            Identity,
            r#"
            SELECT
                user_id as "user_id: user::Id",
                provider_user_id,
                email AS "email: Email",
                is_email_confirmed,
                phone,
                is_phone_confirmed, 
                provider AS "provider: Provider",
                provider_data,
                created_at,
                updated_at
            FROM 
                auth.identities
            WHERE
                user_id = $1;
            "#,
            user_id.as_uuid()
        )
        .fetch_all(&self.db_pool)
        .await?)
    }

    pub async fn add(&self, identity: Identity) -> Result<()> {
        sqlx::query!(
        r#"
        INSERT INTO auth.identities
        (user_id, provider_user_id, email, is_email_confirmed, phone, is_phone_confirmed, provider, provider_data, created_at, updated_at)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
    "#,
        identity.user_id as user::Id,
        identity.provider_user_id,
        identity.email as Email,
        identity.is_email_confirmed,
        identity.phone,
        identity.is_phone_confirmed,
        identity.provider as Provider,
        identity.provider_data,
        identity.created_at,
        identity.updated_at
    ).execute(&self.db_pool).await?;

        Ok(())
    }

    pub async fn get_user_identity(
        &self,
        user_id: &user::Id,
        provider: &Provider,
    ) -> Result<Identity> {
        Ok(sqlx::query_as!(
            Identity,
            r#"
        SELECT
            user_id as "user_id: user::Id",
            provider_user_id,
            email AS "email: Email",
            is_email_confirmed,
            phone,
            is_phone_confirmed,
            provider AS "provider: Provider",
            provider_data,
            created_at,
            updated_at
        FROM 
            auth.identities
        WHERE
            user_id = $1 AND provider = $2;
            "#,
            user_id as &user::Id,
            provider as &Provider
        )
        .fetch_one(&self.db_pool)
        .await?)
    }

    pub async fn get_user_info_from_email_password(
        &self,
        email: Email,
        encrypted_password: EncryptedPassword,
    ) -> Result<AuthenticatedData> {
        Ok(sqlx::query_as!(
                    AuthenticatedData,
                    r#"
                SELECT
                    user_id as "user_id: user::Id", 
                    auth.users.email as "email: Email", 
                    auth.users.phone,
                    provider AS "provider: Provider",
                    provider_data,
                    is_admin
                FROM auth.users
                JOIN auth.identities on auth.users.id = auth.identities.user_id
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
