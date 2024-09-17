use std::{net::SocketAddr, time::Duration};

use anyhow::Context;
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    extract::{ConnectInfo, Query},
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::{
    extract::cookie,
    headers::{self, UserAgent},
    TypedHeader,
};
use http::{header::SET_COOKIE, HeaderMap, StatusCode};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::{Deserialize, Serialize};

use super::AuthRequest;
use super::SESSION_COOKIE_NAME;

use crate::{
    authenticator::Authenticator,
    models::{
        identity::{OrphanIdentity, Provider},
        token::{ACCESS_TOKEN_NAME, REFRESH_TOKEN_NAME},
    },
    routes::AppError,
};

pub async fn google(
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    Extension(store): Extension<MemoryStore>,
) -> Result<impl IntoResponse, AppError> {
    let oauth_client = match authenticator.get_oauth_client(Provider::Google) {
        Some(client) => client,
        None => return Ok((StatusCode::SERVICE_UNAVAILABLE).into_response()),
    };

    let (auth_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new("openid".to_string()))
        .url();

    let mut session = Session::new();

    session
        .insert("csrf_token", &csrf_token)
        .context("failed in inserting CSRF token into session")?;

    let cookie = store
        .store_session(session)
        .await
        .context("failed to store CSRF token session")?
        .context("unexpected error retrieving CSRF cookie value")?;

    let cookie = format!("{SESSION_COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::temporary(auth_url.as_ref())).into_response())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    given_name: String,
    picture: url::Url,
    email: String,
    verified_email: bool,
}

pub async fn callback(
    Query(query): Query<AuthRequest>,
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
    let oauth_client = match authenticator.get_oauth_client(Provider::Google) {
        Some(client) => client,
        None => return Ok((StatusCode::SERVICE_UNAVAILABLE).into_response()),
    };

    super::csrf_token_validation_workflow(&query, &cookies, &store).await?;

    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request to authorization server")?;

    // Fetch user data from google
    let client = reqwest::Client::new();

    let user_data = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<User>()
        .await
        .context("failed to Deserialize to text")?;

    //Ok(format!("{user_data:?}").into_response())

    let user_identity = OrphanIdentity::builder(
        user_data.id.clone(),
        user_data
            .email
            .as_str()
            .try_into()
            .context("Invalid email attached to your discord account.")?,
        Provider::Google,
    )
    .is_email_confirmed(Some(user_data.verified_email))
    .provider_data(
        serde_json::value::to_value(user_data.clone())
            .context("failed to serialize user provider data")?,
    )
    .build();

    let tokens = authenticator
        .oauth_sign_in(user_identity, user_agent.as_str(), &addr.ip())
        .await
        .context("Trouble signing in with oauth")?;
    let access_cookie = cookie::Cookie::build((ACCESS_TOKEN_NAME, tokens.access_jwt))
        .path("/")
        .same_site(cookie::SameSite::Lax)
        .http_only(true)
        .build();
    let refresh_cookie = cookie::Cookie::build((REFRESH_TOKEN_NAME, tokens.refresh_jwt))
        .path("/")
        .same_site(cookie::SameSite::Lax)
        .http_only(true)
        .build();
    //
    let mut headers = HeaderMap::new();
    headers.append(
        SET_COOKIE,
        format!("{};", refresh_cookie,)
            .try_into()
            .context("Trouble injecting cookie.")?,
    );
    headers.append(
        SET_COOKIE,
        format!("{};", access_cookie)
            .try_into()
            .context("Trouble injecting cookie.")?,
    );
    //
    let session_removal_cookie = cookie::Cookie::build((SESSION_COOKIE_NAME, ""))
        .path("/")
        .same_site(cookie::SameSite::Lax)
        .max_age(Duration::from_secs(0).try_into().unwrap())
        .http_only(true)
        .build();
    //
    headers.append(
        SET_COOKIE,
        format!("{};", session_removal_cookie)
            .try_into()
            .context("Trouble injecting cookie.")?,
    );
    //
    Ok((
        headers,
        Redirect::to(authenticator.get_oauth_callback().as_ref()),
    )
        .into_response())
}
