use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
}

impl FromRef<AppState> for () {
    fn from_ref(_: &AppState) -> Self {}
}
