use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Extension,
};
use http::StatusCode;

use crate::authenticator::{self, Authenticator};

#[derive(Debug, serde::Deserialize)]
pub struct Params {
    token: String,
}
pub async fn confirm_email(
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    params: Query<Params>,
) -> impl IntoResponse {
    match authenticator.confirm_email(&params.token).await {
        Ok(_) => "email confirmed".to_string().into_response(),
        Err(authenticator::Error::InvalidToken) => {
            (StatusCode::BAD_REQUEST, "invalid token").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response(),
    };
    Redirect::to("https://www.google.com")
}