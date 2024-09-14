pub mod confirm_email;
pub mod email_auth;
pub mod health;
pub mod oauth;
pub mod reset_password;
pub mod sign_out;

pub fn get_router(
    authenticator: std::sync::Arc<crate::authenticator::Authenticator>,
) -> axum::Router {
    axum::Router::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .route(
            "/sign-up/email",
            axum::routing::post(email_auth::sign_up_email),
        )
        .route(
            "/sign-in/email",
            axum::routing::post(email_auth::sign_in_email),
        )
        .route("/sign-out", axum::routing::get(sign_out::sign_out))
        .route(
            "/confirm-email",
            axum::routing::get(confirm_email::confirm_email),
        )
        .route(
            "/reset-password",
            axum::routing::get(reset_password::start_reset_password)
                .post(reset_password::start_reset_password),
        )
        .nest("/oauth", oauth::get_router())
        //TODO: CHANGE TO APP STATE
        .layer(axum::Extension(authenticator))
        .route("/health", axum::routing::get(health::handler))
}

//pub type HandlerResponse = Result<impl IntoResponse, AppError>
// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
