use crate::api::media::routes::create_media_router;
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn create_main_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Healthy." }))
        .nest("/media", create_media_router())
        .with_state(pool)
}
