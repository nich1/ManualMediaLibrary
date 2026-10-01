use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "upload_status", rename_all = "lowercase")]
pub enum UploadStatus {
    Uploading,
    Finished,
    Failed,
}

pub struct Pagination {
    pub page: u16,
    pub step: u16,
}
