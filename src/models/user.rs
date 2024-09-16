use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{email::Email, identity::Provider, password::EncryptedPassword};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Id(uuid::Uuid);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Id {
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }

    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_trusted_str(str: &str) -> Self {
        Self(Uuid::try_from(str).unwrap())
    }
}
impl From<uuid::Uuid> for Id {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
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
    pub fn builder(email: Email) -> UserBuilder {
        UserBuilder::new(email)
    }
}

#[derive(Debug)]
pub struct UserBuilder {
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

impl UserBuilder {
    pub fn new(email: Email) -> Self {
        Self {
            id: Id::new(),
            email,
            encrypted_password: None,
            email_confirmed_at: None,
            phone: None,
            phone_confirmed_at: None,
            last_sign_in_at: None,
            is_admin: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn email_confirmed_at(mut self, confirmed_at: Option<DateTime<Utc>>) -> Self {
        self.email_confirmed_at = confirmed_at;
        self
    }

    pub fn phone_confirmed_at(mut self, confirmed_at: Option<DateTime<Utc>>) -> Self {
        self.phone_confirmed_at = confirmed_at;
        self
    }
    // Set phone and phone_confirmed_at fields

    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    // Set encrypted_password field
    pub fn encrypted_password(mut self, encrypted_password: EncryptedPassword) -> Self {
        self.encrypted_password = Some(encrypted_password);
        self
    }

    // Set last_sign_in_at field
    pub fn last_sign_in_at(mut self, last_sign_in_at: DateTime<Utc>) -> Self {
        self.last_sign_in_at = Some(last_sign_in_at);
        self
    }

    // Set is_admin field
    pub fn is_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = is_admin;
        self
    }

    // Set updated_at field
    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = updated_at;
        self
    }

    // Finalize and build the User object
    pub fn build(self) -> User {
        User {
            id: self.id,
            email: self.email,
            email_confirmed_at: self.email_confirmed_at,
            phone: self.phone,
            phone_confirmed_at: self.phone_confirmed_at,
            encrypted_password: self.encrypted_password,
            last_sign_in_at: self.last_sign_in_at,
            is_admin: self.is_admin,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
