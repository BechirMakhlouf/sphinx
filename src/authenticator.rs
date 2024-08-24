use crate::{
    config,
    mailer::Mailer,
    models::{
        email::Email,
        identity::{self, Identity},
        password::Password,
        session::{self, Session},
        token::{AuthTokens, TokenClaims, TokenFactory, TokenType},
        user::{self, User},
    },
    repositories::Repository,
};

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    #[error("Provided email is already used: {0}.")]
    EmailAlreadyUsed(String),

    #[error("Provided credentials are invalid.")]
    InvalidCredentials,

    #[error("Internal Error: {0}.")]
    InternalError(String),

    #[error("Can't log in unverified user: {0}.")]
    NotVerifiedAccount(String),

    #[error("Provided Token is invalid.")]
    InvalidToken,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Authenticator {
    repository: Repository,
    token_factory: TokenFactory,
    mailer: Mailer,
    config: config::application::Config,
}

impl Authenticator {
    pub fn new(
        repository: Repository,
        token_factory: TokenFactory,
        mailer: Mailer,
        config: config::application::Config,
    ) -> Self {
        Self {
            repository,
            token_factory,
            mailer,
            config,
        }
    }

    pub async fn email_sign_up(
        &self,
        email: Email,
        password: Password,
        data: serde_json::Value,
    ) -> Result<user::Id> {
        if let Ok(_) = self.repository.user.get_user_by_email(&email).await {
            return Err(Error::EmailAlreadyUsed(email.to_string()));
        }

        let user = User::new(email, Some(password.encrypt()));

        let identity = Identity::builder(
            user.id.clone(),
            user.id.to_string(),
            user.email.clone(),
            identity::Provider::Email,
        )
        .provider_data(data)
        .build();

        let email = identity.email.clone();

        match self.repository.user.add(&user).await {
            Err(sqlx::Error::Database(err)) => match err.kind() {
                sqlx::error::ErrorKind::UniqueViolation => {
                    return Err(Error::EmailAlreadyUsed(email.to_string()))
                }
                _ => return Err(Error::InternalError(err.to_string())),
            },
            Err(err) => return Err(Error::InternalError(err.to_string())),
            Ok(()) => (),
        };

        match self.repository.identity.add(identity).await {
            Err(sqlx::Error::Database(err)) => match err.kind() {
                sqlx::error::ErrorKind::UniqueViolation => {
                    return Err(Error::EmailAlreadyUsed(email.to_string()))
                }
                _ => return Err(Error::InternalError(err.to_string())),
            },
            Err(err) => return Err(Error::InternalError(err.to_string())),
            Ok(()) => (),
        };
        let confirm_email_url = self.create_confirm_email_url(&user.id);

        //TODO: handle errors in email sending
        let _ = self
            .mailer
            .send_confirm_email(&user.email, &confirm_email_url)
            .await;

        Ok(user.id)
    }

    /// creates a reset-password token, persists it in redis cache and sends and email.
    pub async fn intiate_reset_password(&self, email: &Email) -> Result<()> {
        let user = match self.repository.user.get_user_by_email(&email).await {
            Ok(data) => data,
            Err(sqlx::Error::RowNotFound) => return Err(Error::InvalidCredentials),
            Err(err) => return Err(Error::InternalError(err.to_string())),
        };

        let token_id = uuid::Uuid::new_v4().to_string();

        let _ = self
            .repository
            .token
            .store_reset_password_token(
                &user.id,
                &token_id,
                self.config.reset_password.token_expiration_secs,
            )
            .await;

        let token = self.token_factory.create_token(
            TokenType::ResetPassword,
            user.id.as_uuid(),
            Some(self.config.reset_password.token_expiration_secs),
            Some(token_id),
        );

        let reset_password_url = self.create_reset_password_url(&token);
        let v = self
            .mailer
            .send_reset_password(&user.email, &reset_password_url)
            .await;

        Ok(())
    }

    pub fn create_reset_password_url(&self, token: &str) -> url::Url {
        url::Url::try_from(
            format!("{}?token={}", self.config.reset_password.callback, token).as_str(),
        )
        .unwrap()
    }

    pub async fn reset_password(&self, token: &str, new_password: Password) -> Result<bool> {
        let (user_id, token_id) = match self.token_factory.decode_token::<TokenClaims>(token) {
            Ok(token_data) => {
                if token_data.claims.token_type.ne(&TokenType::ResetPassword) {
                    return Err(Error::InvalidToken);
                }
                (
                    user::Id::from_trusted_str(&token_data.claims.sub),
                    token_data.claims.jti.ok_or(Error::InvalidToken)?,
                )
            }
            Err(_) => {
                return Err(Error::InvalidToken);
            }
        };

        //TODO: check if token exists in cache (token is already used)
        match self
            .repository
            .token
            .remove_reset_password_token(&user_id, token_id.as_str())
            .await
        {
            Ok(Some(_)) => (),
            Ok(None) => {
                return Err(Error::InvalidToken);
            }
            Err(err) => {
                return Err(Error::InternalError(err.to_string()));
            }
        };

        match self
            .repository
            .user
            .update_user_password(&user_id, &new_password.encrypt())
            .await
        {
            Ok(result) => Ok(result.rows_affected() == 1),
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }
    fn create_confirm_email_url(&self, user_id: &user::Id) -> url::Url {
        let token = self.token_factory.create_token(
            TokenType::ConfirmEmail,
            user_id.as_uuid(),
            Some(self.config.confirm_email.token_expiration_secs),
            None,
        );

        let url_string = format!(
            "{}?token={}",
            self.config.confirm_email.callback.to_string(),
            token
        );

        url::Url::parse(url_string.as_str()).unwrap()
    }

    pub async fn confirm_email(&self, token: &str) -> Result<bool> {
        let user_id = match self.token_factory.decode_token::<TokenClaims>(token) {
            Ok(token_data) => {
                if token_data.claims.token_type.ne(&TokenType::ConfirmEmail) {
                    return Err(Error::InvalidToken);
                }
                token_data.claims.sub
            }
            Err(_) => {
                return Err(Error::InvalidToken);
            }
        };

        match self
            .repository
            .auth
            .confirm_user_email(&user::Id::from_trusted_str(&user_id))
            .await
        {
            Ok(result) => Ok(result.rows_affected() == 1),
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }

    pub async fn email_sign_in(
        &self,
        email: Email,
        password: Password,
        user_agent: String,
        ip_addr: std::net::IpAddr,
    ) -> Result<AuthTokens> {
        let user = match self.repository.user.get_user_by_email(&email).await {
            Ok(data) => data,
            Err(sqlx::Error::RowNotFound) => return Err(Error::InvalidCredentials),
            Err(err) => return Err(Error::InternalError(err.to_string())),
        };

        if user.email_confirmed_at.is_none() && !self.config.allow_unverified {
            return Err(Error::NotVerifiedAccount(user.email.to_string()));
        }

        match user.encrypted_password {
            Some(encrypted_password) => {
                if !encrypted_password.compare_with(&password) {
                    return Err(Error::InvalidCredentials);
                }
            }
            None => return Err(Error::InvalidCredentials),
        }

        let session = Session::new(user.id.clone(), user_agent, ip_addr);

        //TODO: HANDLE FAILURE HERE
        let _ = self.repository.session.add_session(&session).await;

        let identity = match self
            .repository
            .identity
            .get_user_identity(&user.id, &identity::Provider::Email)
            .await
        {
            Ok(data) => data,
            Err(sqlx::Error::RowNotFound) => return Err(Error::InvalidCredentials),
            Err(err) => return Err(Error::InternalError(err.to_string())),
        };

        let access_jwt = self.token_factory.create_access_token(
            user.id.clone().as_uuid(),
            session.id.clone().as_uuid(),
            Some(identity.provider_data),
        );

        let refresh_jwt = self
            .token_factory
            .create_refresh_token(user.id.clone().as_uuid(), session.id.clone().as_uuid());

        //TODO: HANDLE FAILURE HERE:
        let _ = self
            .repository
            .token
            .store_refresh_token(&user.id, &session.id.as_uuid())
            .await;

        Ok(AuthTokens::new(access_jwt, refresh_jwt))
    }

    pub async fn sign_out(&self, user_id: &user::Id, session_id: &session::Id) -> Result<bool> {
        match self
            .repository
            .token
            .delete_refresh_token(&user_id, &session_id)
            .await
        {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }
}
