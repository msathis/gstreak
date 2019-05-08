use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    data: Vec<CommitLog>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitLog {
    time: DateTime<Utc>,
    message: String,
}

impl CommitLog {
    pub fn new(message: String) -> Self {
        CommitLog {
            time: Utc::now(),
            message,
        }
    }
}