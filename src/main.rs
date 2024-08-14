#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sphinx::init_tracing();

    let config = sphinx::config::get_config();
    sphinx::configure_app(config).await.await.unwrap()
}
