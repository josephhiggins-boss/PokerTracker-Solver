use std::net::SocketAddr;

use solver_api::{build_router, new_app_state};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,solver_api=debug")),
        )
        .init();

    let state = new_app_state();
    let app = build_router(state);

    let addr: SocketAddr = std::env::var("SOLVER_BIND")
        .unwrap_or_else(|_| "127.0.0.1:7878".into())
        .parse()?;
    info!(%addr, "solver-api listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
