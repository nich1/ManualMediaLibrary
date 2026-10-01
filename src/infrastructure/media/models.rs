#[derive(sqlx::FromRow)]
pub struct MediaMetaData {
    pub id: i64,
    pub title: String,
    pub filename: String,
    pub mime_type: String,
    pub duration_ms: i64,
    pub status: String,
}
