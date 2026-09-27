use serde::{Deserialize, Serialize};
use std::time::{Duration};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostMediaMetadata {
    title: String,
    filename: String,
    mime_type: String,
    duration_ms: Duration,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct GetMediaMetaData {
    pub id: u32,
    pub title: String,
    pub filename: String,
    pub mime_type: String,
    pub duration_ms: Duration,
    pub status: UploadStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UploadStatus {
    Uploading,
    Finished,
    Failed,
}