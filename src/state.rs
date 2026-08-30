use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use crate::utils::tripcode::DailySaltCache;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub daily_salt_cache: DailySaltCache,
}

impl FromRef<AppState> for () {
    fn from_ref(_: &AppState) -> Self {}
}
