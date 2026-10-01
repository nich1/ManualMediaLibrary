use crate::application::media::media_handler::{
    delete_media, get_media, get_media_by_id, post_media,
};
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn create_media_router() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_media).post(post_media))
        .route("/{id}", get(get_media_by_id).delete(delete_media))
}
