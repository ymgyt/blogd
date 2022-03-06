#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // find and load environment variables from .env
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env(
            blogd::config::env::LOG,
        ))
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .init();

    let cfg = blogd::Config::load_from_env()?;

    let signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to handle signal");
        tracing::info!("receive SIGINT");
    };

    blogd::inbound::http::serve(cfg.socket_addr()?, signal).await?;

    Ok(())
}
