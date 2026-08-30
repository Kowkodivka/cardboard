use std::net::SocketAddr;

use axum::{
    Json,
    extract::{ConnectInfo, Path, State},
};
use axum_valid::Garde;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
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

#[utoipa::path(
    get,
    path = "/{board_id}/posts/list",
    tag = "boards",
    params(
        ("board_id" = Uuid, Path, description = "Board id"),
    ),
    responses(
        (status = 200, description = "Top-level posts on the board", body = Vec<PostResponse>),
    )
)]
async fn list_top_level(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
) -> AppResult<Json<Vec<PostResponse>>> {
    let posts = Posts::find()
        .filter(posts::Column::BoardId.eq(board_id))
        .filter(posts::Column::ParentPostId.is_null())
        .order_by_desc(posts::Column::CreatedAt)
        .all(&state.db_conn)
        .await?;

    Ok(Json(posts.into_iter().map(Into::into).collect()))
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
    )
)]
async fn create_post(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Garde(Json(payload)): Garde<Json<CreatePostRequest>>,
) -> AppResult<Json<PostResponse>> {
    let author_tripcode =
        generate_tripcode(&state.db_conn, &state.daily_salt_cache, board_id, addr.ip()).await?;

    let post = posts::ActiveModel {
        id: Set(Uuid::new_v4()),
        board_id: Set(board_id),
        parent_post_id: Set(None),
        author_tripcode: Set(author_tripcode),
        content: Set(payload.content),
        ..Default::default()
    };

    let post = post.insert(&state.db_conn).await?;

    Ok(Json(post.into()))
}
