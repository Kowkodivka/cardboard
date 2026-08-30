mod boards;
mod posts;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "boards", description = "Board management"),
        (name = "posts", description = "Posts and replies"),
    )
)]
struct ApiDoc;

pub fn router() -> Router<AppState> {
    let (mut router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/boards", boards::router())
        .nest("/api/posts", posts::router())
        .split_for_parts();

    #[cfg(debug_assertions)]
    {
        use tracing::info;

        router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));
        info!("Docs mounted at /swagger-ui");
    }

    router
}
