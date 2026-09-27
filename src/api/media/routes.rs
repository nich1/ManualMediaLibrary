use axum::{Router, routing::get};
use crate::application::media::media_handler::{get_media, post_media, get_media_by_id};

pub fn create_media_router() -> Router {
    Router::new()
        .route("/", get(get_media).post(post_media))
        .route("/{id}", get(get_media_by_id))

}