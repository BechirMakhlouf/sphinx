use secrecy::ExposeSecret;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const ACCESS_TOKEN_NAME: &str = "access_token";
pub const REFRESH_TOKEN_NAME: &str = "refresh_token";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    ConfirmEmail,
    ResetPassword,
}

pub trait TokenClaimsTrait: Serialize + DeserializeOwned {}

pub struct TokenBuilder {}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccessTokenClaims {
    pub iss: String,
    pub aud: Vec<String>,
    pub sub: uuid::Uuid,
    pub iat: u64,
    pub exp: u64,
    pub session_id: uuid::Uuid,
    pub data: Option<serde_json::Value>,
}

impl TokenClaimsTrait for AccessTokenClaims {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefreshTokenClaims {
    pub iss: String,
    pub aud: Vec<String>,
    pub sub: uuid::Uuid,
    pub iat: u64,
    pub exp: u64,
    pub jti: uuid::Uuid,
}

impl TokenClaimsTrait for RefreshTokenClaims {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenClaims {
    pub iss: String,
    pub token_type: TokenType,
    pub aud: Vec<String>,
    pub sub: String,
    pub jti: Option<String>,
    pub iat: u64,
    pub exp: u64,
}

impl TokenClaimsTrait for TokenClaims {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_jwt: String,
    pub refresh_jwt: String,
}

impl AuthTokens {
    pub fn new(access_jwt: String, refresh_jwt: String) -> Self {
        Self {
            access_jwt,
            refresh_jwt,
        }
    }
}
#[derive(Debug, Clone)]
pub struct TokenFactory {
    iss: String,
    aud: Vec<String>,
    refresh_secret: secrecy::Secret<String>,
    access_secret: secrecy::Secret<String>,
    default_secret: secrecy::Secret<String>,
    access_duration_secs: u64,
    refresh_duration_secs: u64,
    default_duration_secs: u64,
    header: jsonwebtoken::Header,
    validation: jsonwebtoken::Validation,
}

impl TokenFactory {
    pub fn new(jwt_config: crate::config::jwt::Settings) -> Self {
        let mut validation = jsonwebtoken::Validation::default();
        validation.set_audience(&jwt_config.aud);
        Self {
            iss: jwt_config.iss,
            aud: jwt_config.aud,
            access_secret: jwt_config.access_token.secret,
            refresh_secret: jwt_config.refresh_token.secret,
            default_secret: jwt_config.default_token.secret,
            access_duration_secs: jwt_config.access_token.exp_duration_secs,
            refresh_duration_secs: jwt_config.refresh_token.exp_duration_secs,
            default_duration_secs: jwt_config.default_token.exp_duration_secs,
            header: jsonwebtoken::Header::default(),
            validation,
        }
    }

    pub fn create_access_claims(
        &self,
        sub: uuid::Uuid,
        session_id: uuid::Uuid,
        data: Option<serde_json::Value>,
    ) -> AccessTokenClaims {
        AccessTokenClaims {
            aud: self.aud.clone(),
            iss: self.iss.clone(),
            sub,
            iat: jsonwebtoken::get_current_timestamp(),
            exp: jsonwebtoken::get_current_timestamp() + self.access_duration_secs,
            session_id,
            data,
        }
    }

    pub fn create_access_token(
        &self,
        sub: uuid::Uuid,
        session_id: uuid::Uuid,
        data: Option<serde_json::Value>,
    ) -> String {
        let claims = self.create_access_claims(sub, session_id, data);
        self.encode_token(&claims)
    }

    pub fn create_refresh_claims(&self, sub: uuid::Uuid, jti: uuid::Uuid) -> RefreshTokenClaims {
        RefreshTokenClaims {
            aud: self.aud.clone(),
            iss: self.iss.clone(),
            iat: jsonwebtoken::get_current_timestamp(),
            exp: jsonwebtoken::get_current_timestamp() + self.refresh_duration_secs,
            jti,
            sub,
        }
    }
    pub fn create_refresh_token(&self, sub: uuid::Uuid, session_id: uuid::Uuid) -> String {
        let claims = self.create_refresh_claims(sub, session_id);
        self.encode_token(&claims)
    }

    pub fn create_token_claims(
        &self,
        token_type: TokenType,
        sub: uuid::Uuid,
        exp: Option<u64>,
        jti: Option<String>,
    ) -> TokenClaims {
        TokenClaims {
            aud: self.aud.clone(),
            iss: self.iss.clone(),
            iat: jsonwebtoken::get_current_timestamp(),
            exp: jsonwebtoken::get_current_timestamp() + exp.unwrap_or(self.default_duration_secs),
            sub: sub.to_string(),
            token_type,
            jti,
        }
    }

    pub fn create_token(
        &self,
        token_type: TokenType,
        sub: uuid::Uuid,
        exp: Option<u64>,
        jti: Option<String>,
    ) -> String {
        let claims = self.create_token_claims(token_type, sub, exp, jti);
        self.encode_token(&claims)
    }

    pub fn encode_token<T: std::any::Any + TokenClaimsTrait>(&self, claims: &T) -> String {
        use std::any::TypeId;
        let secret = if TypeId::of::<T>() == TypeId::of::<AccessTokenClaims>() {
            self.access_secret.expose_secret().as_bytes()
        } else if TypeId::of::<T>() == TypeId::of::<RefreshTokenClaims>() {
            self.refresh_secret.expose_secret().as_bytes()
        } else {
            self.default_secret.expose_secret().as_bytes()
        };
        jsonwebtoken::encode(
            &self.header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret),
        )
        .expect("Failed to encode jwt.")
    }

    /// Decodes and validates a token
    /// if it's invalid or expired it will return an error.
    pub fn decode_token<T: std::any::Any + TokenClaimsTrait>(
        &self,
        token: &str,
    ) -> Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error> {
        use std::any::TypeId;
        let secret = if TypeId::of::<T>() == TypeId::of::<AccessTokenClaims>() {
            self.access_secret.expose_secret().as_bytes()
        } else if TypeId::of::<T>() == TypeId::of::<RefreshTokenClaims>() {
            self.refresh_secret.expose_secret().as_bytes()
        } else {
            self.default_secret.expose_secret().as_bytes()
        };

        jsonwebtoken::decode::<T>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret),
            &self.validation,
        )
    }
}

pub fn create_jwt<T: Serialize>(claims: &T, secret: &str) -> String {
    let header = jsonwebtoken::Header::default();

    jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to encode jwt.")
}

pub fn parse_jwt<T: DeserializeOwned>(
    jwt: &str,
    secret: &str,
) -> Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error> {
    let validator = jsonwebtoken::Validation::default();

    jsonwebtoken::decode::<T>(
        jwt,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &validator,
    )
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::{AccessTokenClaims, TokenFactory};

    #[test]
    pub fn encode_decode_jwt() {
        let jwt_config: config::jwt::Settings = config::get_config().jwt;
        let token_factory = TokenFactory::new(jwt_config);

        let access_token_claims =
            token_factory.create_access_claims(uuid::Uuid::new_v4(), uuid::Uuid::new_v4(), None);
        let access_token = token_factory.encode_token(&access_token_claims);

        let decoded_access_token = token_factory
            .decode_token::<AccessTokenClaims>(access_token.as_str())
            .unwrap()
            .claims;

        assert_eq!(access_token_claims, decoded_access_token);
    }
}
