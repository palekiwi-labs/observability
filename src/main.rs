use anyhow::Result;
use observability::{config::Config, database::Database, server::create_app};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::from_env()?;
    info!("Starting observability service with config: {:?}", config);

    let database = Database::new(&config.database_url).await?;
    database.migrate().await?;

    let app = create_app(database);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    info!("Server listening on {}:{}", config.host, config.port);

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
