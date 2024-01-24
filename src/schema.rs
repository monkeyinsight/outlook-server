use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHistorySchema {
    pub user: Option<String>,
    pub action: Option<String>,
}
