use crate::application::enums::UploadStatus;
use crate::infrastructure::media::models::MediaMetaData;
use std::time::Duration;

pub async fn create_media_metadata(
    pool: &sqlx::PgPool,
    title: String,
    filename: String,
    mime_type: String,
    duration_ms: Duration,
    status: String,
) -> Result<(MediaMetaData), sqlx::Error> {
    let media_metadata = sqlx::query_as("INSERT INTO mediametadata (title, filename, mime_type, duration_ms, status) VALUES ($1, $2, $3, $4, $5) RETURNING *")
        .bind(title)
        .bind(filename)
        .bind(mime_type)
        .bind(duration_ms)
        .bind(status)
        .fetch_one(pool)
        .await?;
    Ok(media_metadata)
}

pub async fn get_media_metadata(
    pool: &sqlx::PgPool,
    page: u16,
    step: u16,
) -> Result<Vec<MediaMetaData>, sqlx::Error> {
    let limit = i64::from(step);
    let offset = i64::from(page.saturating_sub(1)) * limit;

    let media_metadata = sqlx::query_as::<_, MediaMetaData>(
        "SELECT * FROM mediametadata ORDER BY id LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(media_metadata)
}

pub async fn get_media_metadata_by_id(
    pool: &sqlx::PgPool,
    media_id: i32,
) -> Result<MediaMetaData, sqlx::Error> {
    let media_metadata =
        sqlx::query_as::<_, MediaMetaData>("SELECT * FROM mediametadata WHERE id = $1")
            .bind(media_id)
            .fetch_one(pool)
            .await?;
    Ok(media_metadata)
}

pub async fn delete_media_metadata(
    pool: &sqlx::PgPool,
    media_id: i32,
) -> Result<MediaMetaData, sqlx::Error> {
    let media_metadata =
        sqlx::query_as::<_, MediaMetaData>("DELETE FROM mediametadata WHERE id = $1 RETURNING *")
            .bind(media_id)
            .fetch_one(pool)
            .await?;
    Ok(media_metadata)
}
