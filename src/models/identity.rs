use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use super::{email::Email, user};

#[derive(Debug, Clone, Serialize, sqlx::Type, Deserialize)]
#[sqlx(type_name = "auth_provider", rename_all = "lowercase")]
pub enum Provider {
    Email,
    Google,
    Discord,
    Apple,
    Github,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow, Deserialize)]
pub struct Identity {
    pub user_id: user::Id,
    pub provider_user_id: String,
    pub email: Email,
    pub provider: Provider,

    pub is_email_confirmed: Option<bool>,
    pub phone: Option<String>,
    pub is_phone_confirmed: Option<bool>,
    pub provider_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Identity {
    pub fn builder(
        user_id: user::Id,
        provider_user_id: String,
        email: Email,
        provider: Provider,
    ) -> IdentityBuilder {
        IdentityBuilder::new(user_id, provider_user_id, email, provider)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityBuilder {
    pub user_id: user::Id,
    pub provider_user_id: String,
    pub email: Email,
    pub provider: Provider,

    pub is_email_confirmed: Option<bool>,
    pub phone: Option<String>,
    pub is_phone_confirmed: Option<bool>,
    pub provider_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IdentityBuilder {
    pub fn new(
        user_id: user::Id,
        provider_user_id: String,
        email: Email,
        provider: Provider,
    ) -> Self {
        Self {
            user_id,
            provider_user_id,
            email,
            provider,

            provider_data: serde_json::Value::Null,
            phone: None,
            is_email_confirmed: None,
            is_phone_confirmed: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    pub fn provider_data(mut self, provider_data: serde_json::Value) -> Self {
        self.provider_data = provider_data;
        self
    }
    pub fn phone(mut self, phone: Option<String>) -> Self {
        self.phone = phone;
        self
    }
    pub fn is_email_confirmed(mut self, is_email_confirmed: Option<bool>) -> Self {
        self.is_email_confirmed = is_email_confirmed;
        self
    }
    pub fn is_phone_confirmed(mut self, is_phone_confirmed: Option<bool>) -> Self {
        self.is_phone_confirmed = is_phone_confirmed;
        self
    }
    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = created_at;
        self
    }
    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = updated_at;
        self
    }

    pub fn build(self) -> Identity {
        Identity {
            user_id: self.user_id,
            provider_user_id: self.provider_user_id,
            email: self.email,
            provider: self.provider,
            is_email_confirmed: self.is_email_confirmed,
            phone: self.phone,
            is_phone_confirmed: self.is_phone_confirmed,
            provider_data: self.provider_data,

            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
