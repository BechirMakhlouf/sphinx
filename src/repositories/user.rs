use sqlx::{postgres::PgQueryResult, PgPool};

use crate::models::{
    email::Email,
    password::EncryptedPassword,
    user::{self, User},
};

type Result<T> = std::result::Result<T, sqlx::Error>;

#[derive(Debug, Clone)]
pub struct UserRepository {
    db_pool: PgPool,
}
impl UserRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn update_user_password(
        &self,
        user_id: &user::Id,
        new_password: &EncryptedPassword,
    ) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE auth.users 
            SET 
                encrypted_password = $1
            WHERE
                id = $2;"#,
            new_password as &EncryptedPassword,
            user_id as &user::Id
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT 
                id as "id: user::Id",
                email as "email: Email",
                email_confirmed_at,
                phone,
                phone_confirmed_at,
                encrypted_password as "encrypted_password: EncryptedPassword",
                last_sign_in_at,
                is_admin,
                created_at,
                updated_at
             FROM auth.users;"#
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn get_user_by_id(&self, user_id: user::Id) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT 
                id as "id: user::Id",
                email as "email: Email",
                email_confirmed_at,
                phone,
                phone_confirmed_at,
                encrypted_password as "encrypted_password: EncryptedPassword",
                last_sign_in_at,
                is_admin,
                created_at,
                updated_at
             FROM auth.users
             WHERE id = $1;"#,
            user_id.as_uuid()
        )
        .fetch_optional(&self.db_pool)
        .await
    }

    pub async fn get_user_by_email(&self, email: &Email) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT 
                id as "id: user::Id",
                email as "email: Email",
                email_confirmed_at,
                phone,
                phone_confirmed_at,
                encrypted_password as "encrypted_password: EncryptedPassword",
                last_sign_in_at,
                is_admin,
                created_at,
                updated_at
             FROM auth.users
             WHERE email = $1;"#,
            &email as &Email
        )
        .fetch_one(&self.db_pool)
        .await
    }
    pub async fn add(&self, user: &User) -> Result<()> {
        sqlx::query!(
        r#"
        INSERT INTO auth.users
        (id, email, email_confirmed_at, phone, phone_confirmed_at, encrypted_password, last_sign_in_at, is_admin, created_at, updated_at) 
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
    "#,

            &user.id as &user::Id,
            &user.email as &Email,
            user.email_confirmed_at,
            user.phone,
            user.phone_confirmed_at,
            &user.encrypted_password as &Option<EncryptedPassword>,
            user.last_sign_in_at,
            user.is_admin,
            user.created_at,
            user.updated_at

        ).execute(&self.db_pool).await?;

        Ok(())
    }
}
