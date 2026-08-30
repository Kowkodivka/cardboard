use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::entities::boards;

#[derive(Serialize, ToSchema)]
pub struct BoardResponse {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
}

impl From<boards::Model> for BoardResponse {
    fn from(m: boards::Model) -> Self {
        Self {
            id: m.id,
            slug: m.slug,
            name: m.name,
            description: m.description,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateBoardRequest {
    #[garde(length(min = 1, max = 16))]
    pub slug: String,
    #[garde(length(min = 2, max = 64))]
    pub name: String,
    #[garde(length(max = 2_000))]
    pub description: Option<String>,
}
