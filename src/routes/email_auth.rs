use std::net::SocketAddr;

use ::serde::Deserialize;
use axum::{
    body::Body, extract::ConnectInfo, http::StatusCode, response::Response, Extension, Json,
};
use axum_extra::{extract::cookie, headers::UserAgent, TypedHeader};
use http::header::SET_COOKIE;

use crate::{
    authenticator::Authenticator,
    models::{
        email::Email,
        password::Password,
        token::{ACCESS_TOKEN_NAME, REFRESH_TOKEN_NAME},
    },
};

#[derive(Debug, Clone, Deserialize)]
pub struct RequestBody {
    email: String,
    password: String,
}

pub async fn sign_up_email(
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    Json(body): Json<RequestBody>,
) -> axum::response::Response {
    let authenticator = authenticator.as_ref();

    let email = match Email::try_from(body.email.as_str()) {
        Ok(email) => email,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };
    let password = match Password::try_from(body.password.as_str()) {
        Ok(password) => password,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };

    match authenticator.email_sign_up(email, password).await {
        Ok(user_id) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(user_id.to_string()))
            .unwrap(),
        Err(err) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(err.to_string()))
            .unwrap(),
    }
}

pub async fn sign_in_email(
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(body): Json<RequestBody>,
) -> axum::response::Response {
    let authenticator = authenticator.as_ref();

    let email = match Email::try_from(body.email.as_str()) {
        Ok(email) => email,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };
    let password = match Password::try_from(body.password.as_str()) {
        Ok(password) => password,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };

    match authenticator
        .email_sign_in(email, password, user_agent.to_string(), addr.ip())
        .await
    {
        Ok(auth_tokens) => {
            let access_cookie = cookie::Cookie::build((ACCESS_TOKEN_NAME, auth_tokens.access_jwt))
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .build();
            let refresh_cookie =
                cookie::Cookie::build((REFRESH_TOKEN_NAME, auth_tokens.refresh_jwt))
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .build();

            Response::builder()
                .header(SET_COOKIE, access_cookie.to_string())
                .header(SET_COOKIE, refresh_cookie.to_string())
                .status(StatusCode::OK)
                .body(Body::from(()))
                .unwrap()
        }
        Err(err) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(err.to_string()))
            .unwrap(),
    }
}
