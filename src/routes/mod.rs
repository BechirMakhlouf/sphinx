use axum::Extension;

pub mod auth;
pub mod confirm_email;
pub mod email_auth;
pub mod health;
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
            axum::routing::get(reset_password::start_reset_password),
        )
        .route(
            "/reset-password",
            axum::routing::post(reset_password::reset_password),
        )
        .layer(Extension(authenticator))
        .route("/health", axum::routing::get(health::handler))
}
