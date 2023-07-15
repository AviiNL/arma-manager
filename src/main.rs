use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()?)
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::spawn(async move {
        arma_manager_web::start().await;
    });

    #[cfg(target_os = "windows")]
    steam::Steam::install().await.unwrap();

    // wait for ctrl-c
    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
