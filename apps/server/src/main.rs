use skillcapsule_server::{app, config::Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "skillcapsule_server=debug,tower_http=info".to_string()),
        )
        .init();

    let config = Config::from_env()?;
    let state = app::build_state(config.clone()).await?;
    let router = app::router(state);

    tracing::info!("skillcapsule server listening on {}", config.bind_addr);
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
