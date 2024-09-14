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

//TODO: VALIDATE THE CSRF TOKENS
pub async fn github(
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    Extension(store): Extension<MemoryStore>,
) -> Result<impl IntoResponse, AppError> {
    let oauth_client = match authenticator.get_oauth_client(Provider::Github) {
        Some(client) => client,
        None => return Ok((StatusCode::SERVICE_UNAVAILABLE).into_response()),
    };

    let (auth_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        //.add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
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
    login: Option<String>,
    id: u64,
    node_id: Option<String>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    html_url: Option<String>,
    followers_url: Option<String>,
    following_url: Option<String>,
    gists_url: Option<String>,
    starred_url: Option<String>,
    verified_email: Option<bool>,
    subscriptions_url: Option<String>,
    organizations_url: Option<String>,
    repos_url: Option<String>,
    events_url: Option<String>,
    received_events_url: Option<String>,
    r#type: Option<String>,
    site_admin: Option<bool>,
    name: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    email: Option<String>,
    hireable: Option<bool>,
    bio: Option<String>,
    twitter_username: Option<String>,
    notification_email: Option<String>,
    public_repos: Option<u64>,
    public_gists: Option<u64>,
    followers: Option<u64>,
    following: Option<u64>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

pub async fn callback(
    Query(query): Query<AuthRequest>,
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
    let oauth_client = match authenticator.get_oauth_client(Provider::Github) {
        Some(client) => client,
        None => return Ok((StatusCode::SERVICE_UNAVAILABLE).into_response()),
    };

    super::csrf_token_validation_workflow(&query, &cookies, &store).await?;

    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request to authorization server")?;

    // Fetch user data from github
    let client = reqwest::Client::new();

    let mut user_data = client
        .get("https://api.github.com/user")
        .header("user-agent", "hello")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<User>()
        .await
        .context("failed to Deserialize user_data to json")?;

    let user_emails = client
        .get("https://api.github.com/user/emails")
        .header("user-agent", "hello")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<Vec<GithubEmail>>()
        .await
        .context("failed to Deserialize user_emails to json")?;

    let main_email = user_emails
        .into_iter()
        .find(|email| email.primary && email.verified);

    //TODO: remove the clone please.
    user_data.verified_email = Some(
        main_email
            .clone()
            .with_context(|| "No verified and primary email attached to github account.")?
            .verified
            .clone(),
    );

    user_data.email = main_email.map(|e| e.email);

    let user_identity = OrphanIdentity::builder(
        user_data.id.to_string().clone(),
        user_data
            .email
            .clone()
            .context("a valid email is required.")?
            .as_str()
            .try_into()
            .context("Invalid email attached to your github account.")?,
        Provider::Github,
    )
    .is_email_confirmed(user_data.verified_email)
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
