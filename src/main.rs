use std::env;
use dotenv::dotenv;
use std::time::{Duration};
use axum::{Form, extract::Query, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use crate::UploadStatus::Finished;
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
enum UploadStatus {
    Uploading,
    Finished,
    Failed,
}
#[derive(Debug, Serialize, Deserialize)]
struct MediaMetadataUpload {
    id: u32,
    title: String,
    filename: String,
    mime_type: String,
    duration_ms: Duration,
    status: UploadStatus,
}

fn create_media_router() -> Router {
    Router::new()
        .route("/", get(get_media).post(post_media))
        .route("/{id}", get(get_media_by_id))

}

fn create_main_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Healthy." }))
        .nest("/media", create_media_router())

}

#[tokio::main]
async fn main() {

    dotenv().ok();
    
    let app = create_main_router();
    let port:u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valud u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_media() -> Json<MediaMetadataUpload>{
    let test_val = MediaMetadataUpload {
        id: 3,
        title: String::from("Title Test"),
        filename: String::from("File Name Test"),
        mime_type: String::from("Title Test"),
        duration_ms: Duration::new(5, 0),
        status: Finished,
    };
    Json(test_val)
}



async fn get_media_by_id(id: Query<u32>) -> Json<MediaMetadataUpload>{
    let test_val:MediaMetadataUpload;

    if *id == 0 {
        test_val = MediaMetadataUpload {
            id: 0,
            title: String::from("Title Test"),
            filename: String::from("File Name Test"),
            mime_type: String::from("Title Test"),
            duration_ms: Duration::new(5, 0),
            status: Finished,
        };
    }
    else {
        test_val = MediaMetadataUpload {
            id: 3,
            title: String::from("Title Test 2"),
            filename: String::from("File Name Test 2"),
            mime_type: String::from("Title Test 2"),
            duration_ms: Duration::new(5, 0),
            status: Finished,
        };
    }
    
    Json(test_val)
}


async fn post_media(Form(params): Form<MediaMetadataUpload>) -> Json<MediaMetadataUpload>{
    println!("params: {:?}", params);
    Json(params)
}