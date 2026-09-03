use chrono::{DateTime, FixedOffset};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::entities::posts;

#[derive(Serialize, ToSchema)]
pub struct PostResponse {
    pub id: Uuid,
    pub board_id: Uuid,
    pub root_post_id: Uuid,
    pub parent_post_id: Option<Uuid>,
    pub author_tripcode: String,
    pub content: String,
    pub reply_count: i32,
    pub created_at: DateTime<FixedOffset>,
    pub last_bumped_at: Option<DateTime<FixedOffset>>,
}

impl From<posts::Model> for PostResponse {
    fn from(m: posts::Model) -> Self {
        Self {
            id: m.id,
            board_id: m.board_id,
            root_post_id: m.root_post_id,
            parent_post_id: m.parent_post_id,
            author_tripcode: m.author_tripcode,
            content: m.content,
            reply_count: m.reply_count,
            created_at: m.created_at,
            last_bumped_at: m.last_bumped_at,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreatePostRequest {
    #[garde(length(min = 1, max = 2_000))]
    pub content: String,
}
