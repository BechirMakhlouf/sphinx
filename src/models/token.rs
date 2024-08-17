#![allow(unused)]

use secrecy::ExposeSecret;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::config::jwt;

enum TokenType {
    Refresh,
    Access,
}

trait TokenClaims: Serialize + DeserializeOwned {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccessTokenClaims {
    pub iss: String,
    pub aud: Vec<String>,
    pub sub: uuid::Uuid,
    pub iat: u64,
    pub exp: u64,
    pub data: Option<serde_json::Value>,
}

impl TokenClaims for AccessTokenClaims {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefreshTokenClaims {
    pub iss: String,
    pub aud: Vec<String>,
    pub sub: uuid::Uuid,
    pub iat: u64,
    pub exp: u64,
    pub jti: uuid::Uuid,
}

impl TokenClaims for RefreshTokenClaims {}

struct TokenFactory {
    iss: String,
    aud: Vec<String>,
    refresh_secret: secrecy::Secret<String>,
    access_secret: secrecy::Secret<String>,
    access_duration_secs: u64,
    refresh_duration_secs: u64,
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
            access_duration_secs: jwt_config.access_token.exp_duration_secs,
            refresh_duration_secs: jwt_config.refresh_token.exp_duration_secs,
            header: jsonwebtoken::Header::default(),
            validation,
        }
    }

    fn create_access_claims(
        &self,
        sub: uuid::Uuid,
        data: Option<serde_json::Value>,
    ) -> AccessTokenClaims {
        AccessTokenClaims {
            aud: self.aud.clone(),
            iss: self.iss.clone(),
            sub,
            iat: jsonwebtoken::get_current_timestamp(),
            exp: jsonwebtoken::get_current_timestamp() + self.access_duration_secs,
            data,
        }
    }

    fn create_refresh_claims(&self, sub: uuid::Uuid) -> RefreshTokenClaims {
        RefreshTokenClaims {
            jti: uuid::Uuid::new_v4(),
            aud: self.aud.clone(),
            iss: self.iss.clone(),
            sub,
            iat: jsonwebtoken::get_current_timestamp(),
            exp: jsonwebtoken::get_current_timestamp() + self.refresh_duration_secs,
        }
    }

    pub fn encode_token<T: std::any::Any + TokenClaims>(&self, claims: &T) -> String {
        use std::any::TypeId;
        let secret = if TypeId::of::<T>() == TypeId::of::<AccessTokenClaims>() {
            self.access_secret.expose_secret().as_bytes()
        } else {
            self.refresh_secret.expose_secret().as_bytes()
        };

        jsonwebtoken::encode(
            &self.header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret),
        )
        .expect("Failed to encode jwt.")
    }

    pub fn decode_token<T: std::any::Any + TokenClaims>(
        &self,
        token: &str,
    ) -> jsonwebtoken::TokenData<T> {
        use std::any::TypeId;
        let secret = if TypeId::of::<T>() == TypeId::of::<AccessTokenClaims>() {
            self.access_secret.expose_secret().as_bytes()
        } else {
            self.refresh_secret.expose_secret().as_bytes()
        };

        jsonwebtoken::decode::<T>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret),
            &self.validation,
        )
        .unwrap()
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

        let access_token_claims = token_factory.create_access_claims(uuid::Uuid::new_v4(), None);
        let access_token = token_factory.encode_token(&access_token_claims);

        let decoded_access_token = token_factory
            .decode_token::<AccessTokenClaims>(access_token.as_str())
            .claims;

        assert_eq!(access_token_claims, decoded_access_token);
    }
}
