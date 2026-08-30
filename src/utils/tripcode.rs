use chrono::{Duration, Utc};
use rand::Rng;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use sha2::{Digest, Sha256};
use std::net::IpAddr;
use uuid::Uuid;

use crate::entities::{
    daily_salt,
    prelude::{Boards, DailySalt},
};
use crate::error::{AppError, AppResult};

pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 16];
    rand::rng().fill_bytes(&mut salt);
    salt
}

pub async fn get_current_daily_salt(db: &DatabaseConnection) -> Result<Vec<u8>, sea_orm::DbErr> {
    let existing = DailySalt::find_by_id(1).one(db).await?;

    let needs_rotation = match &existing {
        Some(salt) => Utc::now().signed_duration_since(salt.created_at) > Duration::days(1),
        None => true,
    };

    if needs_rotation {
        let new_value = generate_salt();

        let model = daily_salt::ActiveModel {
            id: Set(1),
            value: Set(new_value.clone()),
            created_at: Set(Utc::now().into()),
        };

        DailySalt::insert(model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(daily_salt::Column::Id)
                    .update_columns([daily_salt::Column::Value, daily_salt::Column::CreatedAt])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(new_value)
    } else {
        Ok(existing.unwrap().value)
    }
}

pub async fn generate_tripcode(
    db_conn: &DatabaseConnection,
    board_id: Uuid,
    ip: IpAddr,
) -> AppResult<String> {
    let board = Boards::find_by_id(board_id)
        .one(db_conn)
        .await?
        .ok_or(AppError::NotFound("board not found"))?;

    let daily_salt = get_current_daily_salt(db_conn).await?;

    let mut hasher = Sha256::new();
    hasher.update(ip.to_string().as_bytes());
    hasher.update(&daily_salt);
    hasher.update(&board.salt);

    let result = hasher.finalize();
    Ok(hex::encode(&result[..8]))
}
