use axum::{
    body::Body,
    extract::Query,
    response::{IntoResponse, Response},
    Extension, Json,
};
use http::StatusCode;

use crate::{
    authenticator::{self, Authenticator},
    models::{email::Email, password::Password},
};

#[derive(Debug, serde::Deserialize)]
pub struct StartResetPasswordParams {
    email: String,
}

pub async fn start_reset_password(
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    params: Query<StartResetPasswordParams>,
) -> impl IntoResponse {
    let email = match Email::try_from(params.email.as_str()) {
        Ok(email) => email,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };

    match authenticator.intiate_reset_password(&email).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(authenticator::Error::InvalidCredentials) => StatusCode::OK.into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ResetPasswordParams {
    pub token: String,
}
#[derive(Debug, serde::Deserialize)]
pub struct ResetPasswordBody {
    new_password: String,
}

pub async fn reset_password(
    Extension(authenticator): Extension<std::sync::Arc<Authenticator>>,
    Query(params): Query<ResetPasswordParams>,
    Json(body): Json<ResetPasswordBody>,
) -> impl IntoResponse {
    let new_password = match Password::try_from(body.new_password.as_str()) {
        Ok(password) => password,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };

    match authenticator
        .reset_password(&params.token, new_password)
        .await
    {
        Ok(_) => "email confirmed".to_string().into_response(),
        Err(authenticator::Error::InvalidToken) => {
            (StatusCode::BAD_REQUEST, "Invalid token").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response(),
    }
}
