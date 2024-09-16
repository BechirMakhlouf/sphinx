use std::fmt::Display;

use super::user;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize, sqlx::Type)]
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
}

pub struct Session {
    pub id: Id,
    pub user_id: user::Id,
    pub user_agent: String,
    pub ip: std::net::IpAddr,
}

impl Session {
    pub fn new(user_id: user::Id, user_agent: String, ip: std::net::IpAddr) -> Self {
        Self {
            id: Id(uuid::Uuid::new_v4()),
            user_id,
            user_agent,
            ip,
        }
    }
}
