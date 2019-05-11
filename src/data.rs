use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    data: Vec<CommitLog>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitLog {
    time: DateTime<Utc>,
    commit: String,
}

impl Data {
    pub fn new() -> Self {
        Data { data: vec![] }
    }

    pub fn add(&mut self, log: CommitLog) {
        self.data.push(log);
    }
}

impl CommitLog {
    pub fn new(message: String) -> Self {
        CommitLog {
            time: Utc::now(),
            commit: message,
        }
    }
}
