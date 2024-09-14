use anyhow::Context;
use async_session::{MemoryStore, SessionStore};
use axum_extra::headers;
use oauth2::CsrfToken;
use serde::Deserialize;

pub mod discord;
pub mod github;
pub mod google;

use super::AppError;

static SESSION_COOKIE_NAME: &str = "SESSION";

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub fn get_router() -> axum::Router {
    axum::Router::new()
        .route("/discord", axum::routing::get(discord::discord))
        .route("/discord/callback", axum::routing::get(discord::callback))
        .route("/google", axum::routing::get(google::google))
        .route("/google/callback", axum::routing::get(google::callback))
        .route("/github", axum::routing::get(github::github))
        .route("/github/callback", axum::routing::get(github::callback))
        .layer(axum::Extension(async_session::MemoryStore::new()))
}

async fn csrf_token_validation_workflow(
    auth_request: &AuthRequest,
    cookies: &headers::Cookie,
    store: &MemoryStore,
) -> Result<(), AppError> {
    // Extract the cookie from the request
    let cookie = cookies
        .get(SESSION_COOKIE_NAME)
        .context("unexpected error getting cookie name")?
        .to_string();

    // Load the session
    let session = store
        .load_session(cookie)
        .await
        .context("failed to load session")?
        .context("Session does not exist")?;

    // Extract the CSRF token from the session
    let stored_csrf_token = session
        .get::<CsrfToken>("csrf_token")
        .context("CSRF token not found in session")?
        .to_owned();

    // Cleanup the CSRF token session
    store
        .destroy_session(session)
        .await
        .context("Failed to destroy old session")?;

    // Validate CSRF token is the same as the one in the auth request
    if *stored_csrf_token.secret() != auth_request.state {
        return Err(anyhow::anyhow!("CSRF token mismatch").into());
    }

    Ok(())
}
