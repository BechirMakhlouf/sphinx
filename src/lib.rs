use axum::{
    extract::{connect_info::IntoMakeServiceWithConnectInfo, ConnectInfo},
    middleware::AddExtension,
};

pub mod authenticator;
pub mod config;
pub mod mailer;
pub mod models;
pub mod oauth;
pub mod password;
pub mod repositories;
pub mod routes;
pub mod services;

pub fn init_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sphinx=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub async fn configure_app(
    config: config::Config,
) -> axum::serve::Serve<
    IntoMakeServiceWithConnectInfo<axum::Router, std::net::SocketAddr>,
    AddExtension<axum::Router, ConnectInfo<std::net::SocketAddr>>,
> {
    let repository = repositories::Repository::new(&config.database, &config.redis, &config.jwt);
    let token_factory = models::token::TokenFactory::new(config.jwt);
    let mailer = mailer::Mailer::new(config.smtp);

    let authenticator = std::sync::Arc::new(authenticator::Authenticator::new(
        repository,
        token_factory,
        mailer,
        config.application.config,
    ));

    let app = routes::get_router(authenticator);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("failed to bind to port");

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
}
