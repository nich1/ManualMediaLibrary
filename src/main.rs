use std::time::{
    Duration,
    Instant
};

use axum::{
    Form,
    extract::Query,
    response::Json,
    routing::get,
    routing::post,
    Router
};
use serde::Deserialize;

use crate::UploadStatus::Finished;

#[derive(Debug, Deserialize)]
enum UploadStatus {
    Uploading,
    Finished,
    Failed,
}
#[derive(Debug, Deserialize)]
struct MediaMetadata {
    id: u32,
    title: String,
    filename: String,
    mime_type: String,
    duration_ms: Duration,
    status: UploadStatus,
    created_at: Instant
}

fn create_media_router() -> Router {
    Router::new()
        .route("/", get(get_media()).post(post_media))
        .route("/{id}", get(get_media_by_id))

}

fn create_main_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Healthy." }))
        .nest("/media", media_routes)

}

#[tokio::main]
async fn main() {

    let app = create_main_router();
        
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_media() -> Json<MediaMetadata>{
    let test_val = MediaMetadata {
        id: 3,
        title: String::from("Title Test"),
        filename: String::from("File Name Test"),
        mime_type: String::from("Title Test"),
        duration_ms: Duration::new(5, 0),
        status: Finished,
        created_at: Instant::now()
    };
    Json(test_val)
}



async fn get_media_by_id(id: Query<u32>) -> Json<MediaMetadata>{
    let mut test_val:MediaMetadata;

    if (id == 0) {
        test_val = MediaMetadata {
            id: 0,
            title: String::from("Title Test"),
            filename: String::from("File Name Test"),
            mime_type: String::from("Title Test"),
            duration_ms: Duration::new(5, 0),
            status: Finished,
            created_at: Instant::now()
        };
    }
    else {
        test_val = MediaMetadata {
            id: 3,
            title: String::from("Title Test 2"),
            filename: String::from("File Name Test 2"),
            mime_type: String::from("Title Test 2"),
            duration_ms: Duration::new(5, 0),
            status: Finished,
            created_at: Instant::now()
        };
    }
    
    Json(test_val)
}


async fn post_media(Form(params): Form<MediaMetadata>) -> Json<MediaMetadata>{
    println!("params: {:?}", params);
    Json(params)
}