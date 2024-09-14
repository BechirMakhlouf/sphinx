use sqlx::{postgres::PgQueryResult, PgPool};

use crate::models::{
    email::Email,
    identity::{Identity, Provider},
    user::{self},
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
        sqlx::query_as!(
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
        .await
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
        provider: &Provider,
        provider_user_id: &str,
    ) -> Result<Identity> {
        sqlx::query_as!(
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
            provider = $1 AND provider_user_id = $2;
            "#,
            provider as &Provider,
            provider_user_id
        )
        .fetch_one(&self.db_pool)
        .await
    }

    pub async fn update_identity(&self, identity: &Identity) -> Result<PgQueryResult> {
        sqlx::query!(
            "
        UPDATE auth.identities
        SET 
            provider_data = $1,
            email = $2,
            is_email_confirmed = $3,
            phone = $4,
            is_phone_confirmed = $5
        WHERE 
            provider = $6 AND provider_user_id = $7
        ",
            identity.provider_data,
            &identity.email as &Email,
            identity.is_email_confirmed,
            identity.phone,
            identity.is_phone_confirmed,
            &identity.provider as &Provider,
            identity.provider_user_id,
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn upsert_identity(&self, identity: &Identity) -> Result<PgQueryResult> {
        sqlx::query!(
            "
            INSERT INTO auth.identities (
                user_id, 
                provider, 
                provider_user_id, 
                provider_data, 
                email, 
                is_email_confirmed, 
                phone, 
                is_phone_confirmed, 
                created_at, 
                updated_at
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (provider, provider_user_id) 
            DO UPDATE SET 
                provider_data = EXCLUDED.provider_data,
                email = EXCLUDED.email,
                is_email_confirmed = EXCLUDED.is_email_confirmed,
                phone = EXCLUDED.phone,
                is_phone_confirmed = EXCLUDED.is_phone_confirmed
        ",
            &identity.user_id as &user::Id,
            &identity.provider as &Provider,
            identity.provider_user_id,
            identity.provider_data,
            &identity.email as &Email,
            identity.is_email_confirmed,
            identity.phone,
            identity.is_phone_confirmed,
            identity.created_at,
            identity.updated_at
        )
        .execute(&self.db_pool)
        .await
    }
}
