mod entities;
mod error;
mod models;
mod routes;
mod state;
mod utils;

use std::{env, net::SocketAddr};

use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::{state::AppState, utils::tripcode::DailySaltCache};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::from("info")))
        .init();

    let db_url = env::var("CARDBOARD_DATABASE_URL")?;
    let db_conn = Database::connect(db_url).await?;
    Migrator::up(&db_conn, None).await?;

    let app = routes::router().with_state(AppState {
        db_conn,
        daily_salt_cache: DailySaltCache::new(),
    });

    let addr = env::var("CARDBOARD_ADDR")?;
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
