pub mod config;
pub mod routes;

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
) -> axum::serve::Serve<axum::Router, axum::Router> {
    let app = axum::Router::new()
        .with_state(config.database.get_db_pool())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .route("/health", axum::routing::get(routes::health::handler));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("failed to bind to port");

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
}
