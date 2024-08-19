use crate::{
    mailer::Mailer,
    models::{
        email::Email,
        identity::{self, Identity},
        password::Password,
        token::TokenFactory,
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
}

type Result<T> = std::result::Result<T, Error>;

pub struct Authenticator {
    repository: Repository,
    token_factory: TokenFactory,
    mailer: Mailer,
    allow_unverified: bool,
}

impl Authenticator {
    // email sign-up,in;
    pub async fn email_sign_up(
        &self,
        email: Email,
        password: Password,
        data: serde_json::Value,
    ) -> Result<user::Id> {
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

        Ok(user.id)
    }
    // oauth sign-up,in;
    // sign-in magic-link;
    // signout
}
