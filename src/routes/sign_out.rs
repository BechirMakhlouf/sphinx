use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

use crate::models::token::{ACCESS_TOKEN_NAME, REFRESH_TOKEN_NAME};

pub async fn sign_out(cookie_jar: CookieJar) -> impl IntoResponse {
    cookie_jar
        .remove(REFRESH_TOKEN_NAME)
        .remove(ACCESS_TOKEN_NAME)
}
