use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct HistoryModel {
    pub id: Uuid,
    pub topic: String,
    pub user: String,
    pub action: String,
    pub date: Option<chrono::DateTime<chrono::Utc>>,
}
