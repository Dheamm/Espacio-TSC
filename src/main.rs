mod alias;
mod db;
mod errors;
mod handlers;
mod models;
mod router;
mod state;
mod user;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "tsc_app=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:tsc.db".to_string());

    let pool = db::create_pool(&database_url).await?;
    db::run_migrations(&pool).await?;

    let state = state::AppState::new(pool);
    let app = router::build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
