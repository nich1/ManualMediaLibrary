use axum::{response::Json, Form, extract::Query};
use std::time::Duration;
use crate::api::media::models::{GetMediaMetaData, PostMediaMetadata, UploadStatus};


pub async fn get_media() -> Json<GetMediaMetaData>{
    let test_val = GetMediaMetaData {
        id: 0,
        title: String::from("Title Test"),
        filename: String::from("File Name Test"),
        mime_type: String::from("Title Test"),
        duration_ms: Duration::new(5, 0),
        status: UploadStatus::Finished
    };
    Json(test_val)
}



pub async fn get_media_by_id(id: Query<u32>) -> Json<GetMediaMetaData>{
    let test_val:GetMediaMetaData;

    if *id == 0 {
        test_val = GetMediaMetaData {
            id: 0,
            title: String::from("Title Test"),
            filename: String::from("File Name Test"),
            mime_type: String::from("Title Test"),
            duration_ms: Duration::new(5, 0),
            status: UploadStatus::Finished,
        };
    }
    else {
        test_val = GetMediaMetaData {
            id: 3,
            title: String::from("Title Test 2"),
            filename: String::from("File Name Test 2"),
            mime_type: String::from("Title Test 2"),
            duration_ms: Duration::new(5, 0),
            status: UploadStatus::Finished,
        };
    }
    
    Json(test_val)
}


pub async fn post_media(Form(params): Form<PostMediaMetadata>) -> Json<PostMediaMetadata>{
    println!("params: {:?}", params);
    Json(params)
}