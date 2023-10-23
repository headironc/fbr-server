use axum::Server;
use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

use fbr_server::{routes::router, state::State};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or("fbr_server=debug,tower_http=debug".into()),
        )
        .with(fmt::layer())
        .init();

    let state = State::instance().await;
    let config = state.config();

    let router = router();

    info!("Listening on http://{}", config.addr());

    Server::bind(&config.addr())
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
