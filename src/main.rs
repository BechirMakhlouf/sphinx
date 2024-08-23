#[tokio::main]
async fn main() {
    sphinx::init_tracing();
    let config = sphinx::config::get_config();
    sphinx::configure_app(config).await.await.unwrap()
}
