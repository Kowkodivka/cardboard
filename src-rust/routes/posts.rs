use axum::{
    Json,
    extract::{ConnectInfo, Path, Query, State},
};
use axum_valid::Garde;
use chrono::{DateTime, FixedOffset, Utc};
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, ExprTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait,
};
use serde::Deserialize;
use std::net::SocketAddr;
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    entities::{posts, prelude::Posts},
    error::{AppError, AppResult},
    models::{CreatePostRequest, PostResponse},
    state::AppState,
    utils::salt::generate_tripcode,
};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(list_replies, create_reply))
}

#[derive(Deserialize, IntoParams)]
struct ListRepliesQuery {
    after: Option<DateTime<FixedOffset>>,
    #[serde(default = "default_limit")]
    limit: u64,
}

fn default_limit() -> u64 {
    50
}

#[utoipa::path(
    get,
    path = "/{post_id}/replies/list",
    tag = "posts",
    params(
        ("post_id" = Uuid, Path, description = "Any post id within the thread"),
        ListRepliesQuery,
    ),
    responses(
        (status = 200, description = "Page of posts in the thread", body = Vec<PostResponse>),
        (status = 404, description = "Post not found"),
    )
)]
async fn list_replies(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
    Query(query): Query<ListRepliesQuery>,
) -> AppResult<Json<Vec<PostResponse>>> {
    let anchor = Posts::find_by_id(post_id)
        .one(&state.db_conn)
        .await?
        .ok_or(AppError::NotFound("post not found"))?;

    let limit = std::cmp::min(query.limit, 50);

    let mut select = Posts::find()
        .filter(posts::Column::RootPostId.eq(anchor.root_post_id))
        .order_by_asc(posts::Column::CreatedAt)
        .limit(limit);

    if let Some(after) = query.after {
        select = select.filter(posts::Column::CreatedAt.gt(after));
    }

    let thread = select.all(&state.db_conn).await?;

    Ok(Json(thread.into_iter().map(Into::into).collect()))
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

    let author_tripcode = generate_tripcode(
        &state.db_conn,
        &state.daily_salt_cache,
        parent.board_id,
        addr.ip(),
    )
    .await?;

    let reply = insert_reply(&txn, &parent, author_tripcode, payload.content).await?;

    bump_thread(&txn, parent.root_post_id).await?;

    txn.commit().await?;

    Ok(Json(reply.into()))
}

async fn insert_reply(
    txn: &DatabaseTransaction,
    parent: &posts::Model,
    author_tripcode: String,
    content: String,
) -> AppResult<posts::Model> {
    let reply = posts::ActiveModel {
        id: Set(Uuid::new_v4()),
        board_id: Set(parent.board_id),
        root_post_id: Set(parent.root_post_id),
        parent_post_id: Set(Some(parent.id)),
        author_tripcode: Set(author_tripcode),
        content: Set(content),
        ..Default::default()
    };

    Ok(reply.insert(txn).await?)
}

async fn bump_thread(txn: &DatabaseTransaction, root_post_id: Uuid) -> AppResult<()> {
    let now: DateTime<FixedOffset> = Utc::now().fixed_offset();

    let result = posts::Entity::update_many()
        .col_expr(
            posts::Column::ReplyCount,
            Expr::col(posts::Column::ReplyCount).add(1),
        )
        .col_expr(posts::Column::LastBumpedAt, Expr::value(now))
        .filter(posts::Column::Id.eq(root_post_id))
        .exec(txn)
        .await?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound("thread root post not found"));
    }

    Ok(())
}
