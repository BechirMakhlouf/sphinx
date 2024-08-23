use auth::AuthRepository;
use identity::IdentityRepository;
use session::SessionRepository;
use token::TokenRepository;
use user::UserRepository;

use crate::config;

pub mod auth;
pub mod identity;
pub mod session;
pub mod token;
pub mod user;

pub type Result<T> = std::result::Result<T, sqlx::Error>;

#[derive(Debug, Clone)]
pub struct Repository {
    pub auth: AuthRepository,
    pub identity: IdentityRepository,
    pub session: SessionRepository,
    pub user: UserRepository,
    pub token: TokenRepository,
}

impl Repository {
    pub fn new(
        db_settings: &config::database::Settings,
        redis_settings: &config::redis::Settings,
        token_settings: &config::jwt::Settings,
    ) -> Self {
        let db_pool = db_settings.get_db_pool();
        let redis_client = redis_settings.get_client();

        Self {
            auth: AuthRepository::new(db_pool.clone()),
            identity: IdentityRepository::new(db_pool.clone()),
            session: SessionRepository::new(db_pool.clone()),
            user: UserRepository::new(db_pool.clone()),
            token: TokenRepository::new(
                redis_client,
                token_settings.refresh_token.exp_duration_secs,
            ),
        }
    }
}
