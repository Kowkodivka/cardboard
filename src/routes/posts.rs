use axum::{
    Json,
    extract::{ConnectInfo, Path, State},
};
use axum_valid::Garde;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use std::net::SocketAddr;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    entities::{posts, prelude::Posts},
    error::{AppError, AppResult},
    models::{CreatePostRequest, PostResponse},
    state::AppState,
    utils::tripcode::generate_tripcode,
};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(list_replies, create_reply))
}

#[utoipa::path(
    get,
    path = "/{post_id}/replies/list",
    tag = "posts",
    params(
        ("post_id" = Uuid, Path, description = "Parent post id"),
    ),
    responses(
        (status = 200, description = "Replies to the post", body = Vec<PostResponse>),
    )
)]
async fn list_replies(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> AppResult<Json<Vec<PostResponse>>> {
    let replies = Posts::find()
        .filter(posts::Column::ParentPostId.eq(post_id))
        .order_by_asc(posts::Column::CreatedAt)
        .all(&state.db_conn)
        .await?;

    Ok(Json(replies.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    post,
    path = "/{post_id}/replies/create",
    tag = "posts",
    params(
        ("post_id" = Uuid, Path, description = "Parent post id"),
    ),
    request_body = CreatePostRequest,
    responses(
        (status = 200, description = "Reply created", body = PostResponse),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Parent post not found"),
    )
)]
async fn create_reply(
    State(state): State<AppState>,
    Path(parent_post_id): Path<Uuid>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Garde(Json(payload)): Garde<Json<CreatePostRequest>>,
) -> AppResult<Json<PostResponse>> {
    let txn = state.db_conn.begin().await?;

    let parent = Posts::find_by_id(parent_post_id)
        .one(&txn)
        .await?
        .ok_or(AppError::NotFound("parent post not found"))?;

    let author_tripcode = generate_tripcode(&state.db_conn, parent.board_id, addr.ip()).await?;

    let reply = posts::ActiveModel {
        id: Set(Uuid::new_v4()),
        board_id: Set(parent.board_id),
        parent_post_id: Set(Some(parent_post_id)),
        author_tripcode: Set(author_tripcode),
        content: Set(payload.content),
        ..Default::default()
    };

    let reply = reply.insert(&txn).await?;

    let mut parent: posts::ActiveModel = parent.into();
    parent.reply_count = Set(parent.reply_count.unwrap() + 1);
    parent.update(&txn).await?;

    txn.commit().await?;

    Ok(Json(reply.into()))
}
