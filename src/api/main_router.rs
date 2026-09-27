use axum::{Router, routing::get};
use crate::api::media::routes::create_media_router;
pub fn create_main_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Healthy." }))
        .nest("/media", create_media_router())

}