use std::net::SocketAddr;

use axum::{
    Json,
    extract::{ConnectInfo, Path, Query, State},
};
use axum_valid::Garde;
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    entities::{boards, posts, prelude::*},
    error::{AppError, AppResult},
    models::{BoardResponse, CreateBoardRequest, CreatePostRequest, PostResponse},
    state::AppState,
    utils::tripcode::{generate_salt, generate_tripcode},
};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_boards, create_board))
        .routes(routes!(get_board))
        .routes(routes!(list_top_level, create_post))
}

#[utoipa::path(
    get,
    path = "/list",
    tag = "boards",
    responses(
        (status = 200, description = "List of all boards", body = Vec<BoardResponse>)
    )
)]
async fn list_boards(State(state): State<AppState>) -> AppResult<Json<Vec<BoardResponse>>> {
    let boards = Boards::find().all(&state.db_conn).await?;

    Ok(Json(boards.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    post,
    path = "/create",
    tag = "boards",
    request_body = CreateBoardRequest,
    responses(
        (status = 200, description = "Board created", body = BoardResponse),
        (status = 400, description = "Validation error"),
    )
)]
async fn create_board(
    State(state): State<AppState>,
    Garde(Json(payload)): Garde<Json<CreateBoardRequest>>,
) -> AppResult<Json<BoardResponse>> {
    let salt = generate_salt();

    let board = boards::ActiveModel {
        id: Set(Uuid::new_v4()),
        slug: Set(payload.slug),
        name: Set(payload.name),
        description: Set(payload.description),
        salt: Set(salt),
        ..Default::default()
    };

    let board = board.insert(&state.db_conn).await?;

    Ok(Json(board.into()))
}

#[utoipa::path(
    get,
    path = "/{slug}/get",
    tag = "boards",
    params(
        ("slug" = String, Path, description = "Board slug"),
    ),
    responses(
        (status = 200, description = "Board found", body = BoardResponse),
        (status = 404, description = "Board not found"),
    )
)]
async fn get_board(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<BoardResponse>> {
    let board = Boards::find()
        .filter(boards::Column::Slug.eq(slug))
        .one(&state.db_conn)
        .await?
        .ok_or(AppError::NotFound("board not found"))?;

    Ok(Json(board.into()))
}

#[derive(Deserialize, IntoParams)]
struct ListTopLevelQuery {
    before: Option<DateTime<FixedOffset>>,
    #[serde(default = "default_limit")]
    limit: u64,
}

fn default_limit() -> u64 {
    50
}

#[utoipa::path(
    get,
    path = "/{board_id}/posts/list",
    tag = "boards",
    params(
        ("board_id" = Uuid, Path, description = "Board id"),
        ListTopLevelQuery,
    ),
    responses(
        (status = 200, description = "Page of threads on the board, newest bump first", body = Vec<PostResponse>),
    )
)]
async fn list_top_level(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
    Query(query): Query<ListTopLevelQuery>,
) -> AppResult<Json<Vec<PostResponse>>> {
    let limit = query.limit.min(100);

    let mut select = Posts::find()
        .filter(posts::Column::BoardId.eq(board_id))
        .filter(posts::Column::ParentPostId.is_null())
        .order_by_desc(posts::Column::LastBumpedAt)
        .limit(limit);

    if let Some(before) = query.before {
        select = select.filter(posts::Column::LastBumpedAt.lt(before));
    }

    let threads = select.all(&state.db_conn).await?;

    Ok(Json(threads.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    post,
    path = "/{board_id}/posts/create",
    tag = "boards",
    params(
        ("board_id" = Uuid, Path, description = "Board id"),
    ),
    request_body = CreatePostRequest,
    responses(
        (status = 200, description = "Post created", body = PostResponse),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Board not found"),
    )
)]
async fn create_post(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Garde(Json(payload)): Garde<Json<CreatePostRequest>>,
) -> AppResult<Json<PostResponse>> {
    Boards::find_by_id(board_id)
        .one(&state.db_conn)
        .await?
        .ok_or(AppError::NotFound("board not found"))?;

    let author_tripcode =
        generate_tripcode(&state.db_conn, &state.daily_salt_cache, board_id, addr.ip()).await?;

    let post_id = Uuid::new_v4();

    let post = posts::ActiveModel {
        id: Set(post_id),
        board_id: Set(board_id),
        root_post_id: Set(post_id),
        parent_post_id: Set(None),
        author_tripcode: Set(author_tripcode),
        content: Set(payload.content),
        last_bumped_at: Set(Some(Utc::now().fixed_offset())),
        ..Default::default()
    };

    let post = post.insert(&state.db_conn).await?;

    Ok(Json(post.into()))
}
