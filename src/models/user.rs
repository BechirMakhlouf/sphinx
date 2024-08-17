use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{email::Email, password::EncryptedPassword};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow, Deserialize)]
pub struct User {
    id: Id,

    email: Email,
    email_confirmed_at: Option<DateTime<Utc>>,

    phone: Option<String>,
    phone_confirmed_at: Option<DateTime<Utc>>,

    encrypted_password: Option<EncryptedPassword>,

    last_sign_in_at: Option<DateTime<Utc>>,
    is_admin: bool,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
