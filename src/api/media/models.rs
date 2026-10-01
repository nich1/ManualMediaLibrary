use crate::application::enums::UploadStatus;
use crate::infrastructure::media::models::MediaMetaData as MediaRow;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostMediaMetadata {
    pub title: String,
    pub filename: String,
    pub mime_type: String,
    pub duration_ms: Duration,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaMetaData {
    pub id: u32,
    pub title: String,
    pub filename: String,
    pub mime_type: String,
    pub duration_ms: Duration,
    pub status: UploadStatus,
}

#[derive(Debug)]
pub enum MediaConversionError {
    InvalidId(i64),
    InvalidDuration(i64),
    UnknownStatus(String),
}

impl TryFrom<MediaRow> for MediaMetaData {
    type Error = MediaConversionError;

    fn try_from(row: MediaRow) -> Result<Self, Self::Error> {
        let id = u32::try_from(row.id).map_err(|_| MediaConversionError::InvalidId(row.id))?;

        let milliseconds = u64::try_from(row.duration_ms)
            .map_err(|_| MediaConversionError::InvalidDuration(row.duration_ms))?;

        let status = match row.status.as_str() {
            "uploading" => UploadStatus::Uploading,
            "finished" => UploadStatus::Finished,
            "failed" => UploadStatus::Failed,
            _ => return Err(MediaConversionError::UnknownStatus(row.status)),
        };

        Ok(Self {
            id,
            title: row.title,
            filename: row.filename,
            mime_type: row.mime_type,
            duration_ms: Duration::from_millis(milliseconds),
            status,
        })
    }
}

pub struct UploadMediaMetaData {
    pub title: String,
    pub filename: String,
    pub mime_type: String,
    pub duration_ms: Duration,
}
