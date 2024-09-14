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
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("read:email".to_string()))
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
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    site_admin: bool,
    name: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    email: Option<String>,
    verified_email: Option<bool>,
    hireable: Option<bool>,
    bio: Option<String>,
    twitter_username: Option<String>,
    notification_email: Option<String>,
    public_repos: u32,
    public_gists: u32,
    followers: u32,
    following: u32,
    created_at: String,
    updated_at: String,
    private_gists: u32,
    total_private_repos: u32,
    owned_private_repos: u32,
    disk_usage: u64,
    collaborators: u32,
    two_factor_authentication: bool,
}

#[derive(Debug, Deserialize)]
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
        //.json::<User>()
        .text()
        .await
        .context("failed to Deserialize to json")?;

    let user_emails = client
        .get("https://api.github.com/user/emails")
        .header("user-agent", "hello")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        //.json::<Vec<GithubEmail>>()
        .text()
        .await
        .context("failed to Deserialize to json")?;

    println!("hello world");
    println!("emails: {user_emails:?}");

    //let main_email = user_emails
    //    .into_iter()
    //    .find(|email| email.primary && email.verified);
    //
    //user_data.verified_email = match &main_email {
    //    Some(main_email) => Some(main_email.verified),
    //    None => Some(false),
    //};
    //
    //user_data.email = main_email.map(|e| e.email);

    Ok(format!("{user_data:?}").into_response())

    //let user_identity = OrphanIdentity::builder(
    //    user_data.id.to_string().clone(),
    //    user_data
    //        .email
    //        .clone()
    //        .context("a valid email is required.")?
    //        .as_str()
    //        .try_into()
    //        .context("Invalid email attached to your github account.")?,
    //    Provider::Github,
    //)
    //.is_email_confirmed(user_data.verified_email)
    //.provider_data(
    //    serde_json::value::to_value(user_data.clone())
    //        .context("failed to serialize user provider data")?,
    //)
    //.build();

    //let tokens = authenticator
    //    .oauth_sign_in(user_identity, user_agent.as_str(), &addr.ip())
    //    .await
    //    .context("Trouble signing in with oauth")?;

    //let access_cookie = cookie::Cookie::build((ACCESS_TOKEN_NAME, tokens.access_jwt))
    //    .path("/")
    //    .same_site(cookie::SameSite::Lax)
    //    .http_only(true)
    //    .build();
    //let refresh_cookie = cookie::Cookie::build((REFRESH_TOKEN_NAME, tokens.refresh_jwt))
    //    .path("/")
    //    .same_site(cookie::SameSite::Lax)
    //    .http_only(true)
    //    .build();
    ////
    //let mut headers = HeaderMap::new();
    //headers.append(
    //    SET_COOKIE,
    //    format!("{};", refresh_cookie,)
    //        .try_into()
    //        .context("Trouble injecting cookie.")?,
    //);
    //headers.append(
    //    SET_COOKIE,
    //    format!("{};", access_cookie)
    //        .try_into()
    //        .context("Trouble injecting cookie.")?,
    //);
    ////
    //let session_removal_cookie = cookie::Cookie::build((SESSION_COOKIE_NAME, ""))
    //    .path("/")
    //    .same_site(cookie::SameSite::Lax)
    //    .max_age(Duration::from_secs(0).try_into().unwrap())
    //    .http_only(true)
    //    .build();
    ////
    //headers.append(
    //    SET_COOKIE,
    //    format!("{};", session_removal_cookie)
    //        .try_into()
    //        .context("Trouble injecting cookie.")?,
    //);
    ////
    //Ok((
    //    headers,
    //    Redirect::to(authenticator.get_oauth_callback().as_ref()),
    //)
    //    .into_response())
}
