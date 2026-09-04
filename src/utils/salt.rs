use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use sea_orm::{DatabaseConnection, EntityTrait, Set, sea_query::OnConflict};
use sha2::{Digest, Sha256};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::entities::{
    daily_salt,
    prelude::{Boards, DailySalt},
};
use crate::error::{AppError, AppResult};

const SALT_ROTATION_PERIOD: Duration = Duration::days(1);
const SALT_LEN: usize = 16;
const TRIPCODE_LEN: usize = 8;
const DAILY_SALT_ROW_ID: i32 = 1;

#[derive(Clone, Default)]
pub struct DailySaltCache {
    inner: Arc<RwLock<Option<CachedSalt>>>,
}

#[derive(Clone)]
struct CachedSalt {
    value: Vec<u8>,
    created_at: DateTime<Utc>,
}

impl CachedSalt {
    fn is_stale(&self) -> bool {
        Utc::now().signed_duration_since(self.created_at) > SALT_ROTATION_PERIOD
    }
}

impl DailySaltCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get(&self, db_conn: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        if let Some(cached) = self.inner.read().await.as_ref() {
            if !cached.is_stale() {
                return Ok(cached.value.clone());
            }
        }

        let mut guard = self.inner.write().await;
        if let Some(cached) = guard.as_ref() {
            if !cached.is_stale() {
                return Ok(cached.value.clone());
            }
        }

        let refreshed = fetch_or_rotate_daily_salt(db_conn).await?;
        let value = refreshed.value.clone();
        *guard = Some(refreshed);

        Ok(value)
    }
}

pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; SALT_LEN];
    rand::rng().fill_bytes(&mut salt);
    salt
}

async fn fetch_or_rotate_daily_salt(db_conn: &DatabaseConnection) -> anyhow::Result<CachedSalt> {
    let existing = DailySalt::find_by_id(DAILY_SALT_ROW_ID)
        .one(db_conn)
        .await?;

    if let Some(salt) = existing {
        let created_at = salt.created_at.into();
        if Utc::now().signed_duration_since(created_at) <= SALT_ROTATION_PERIOD {
            return Ok(CachedSalt {
                value: salt.value,
                created_at,
            });
        }
    }

    let value = generate_salt();
    let created_at = Utc::now();

    let model = daily_salt::ActiveModel {
        id: Set(DAILY_SALT_ROW_ID),
        value: Set(value.clone()),
        created_at: Set(created_at.into()),
    };

    DailySalt::insert(model)
        .on_conflict(
            OnConflict::column(daily_salt::Column::Id)
                .update_columns([daily_salt::Column::Value, daily_salt::Column::CreatedAt])
                .to_owned(),
        )
        .exec(db_conn)
        .await?;

    Ok(CachedSalt { value, created_at })
}

pub async fn generate_tripcode(
    db_conn: &DatabaseConnection,
    salt_cache: &DailySaltCache,
    board_id: Uuid,
    ip: IpAddr,
) -> AppResult<String> {
    let board = Boards::find_by_id(board_id)
        .one(db_conn)
        .await?
        .ok_or(AppError::NotFound("board not found"))?;

    let daily_salt = salt_cache.get(db_conn).await?;

    let mut hasher = Sha256::new();
    hasher.update(ip.to_string().as_bytes());
    hasher.update(&daily_salt);
    hasher.update(&board.salt);

    let result = hasher.finalize();
    Ok(hex::encode(&result[..TRIPCODE_LEN]))
}
