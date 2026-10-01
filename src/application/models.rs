use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u16,
    pub step: u16,
}
