use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{email::Email, identity::Provider, password::EncryptedPassword};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Id(uuid::Uuid);

impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.into()
    }
}
impl Id {
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow, Deserialize)]
pub struct User {
    pub id: Id,
    pub email: Email,
    pub email_confirmed_at: Option<DateTime<Utc>>,

    pub phone: Option<String>,
    pub phone_confirmed_at: Option<DateTime<Utc>>,

    pub encrypted_password: Option<EncryptedPassword>,

    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub is_admin: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthenticatedData {
    pub user_id: Id,
    pub email: Email,
    pub phone: Option<String>,
    pub provider: Provider,
    pub provider_data: serde_json::Value,
    pub is_admin: bool,
}

impl User {
    pub fn new(email: Email, password: Option<EncryptedPassword>) -> Self {
        Self {
            id: Id::new(),
            email,
            encrypted_password: password,
            email_confirmed_at: None,
            phone: None,
            phone_confirmed_at: None,
            last_sign_in_at: None,
            is_admin: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
