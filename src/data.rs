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
    branch: String,
}

impl Data {
    pub fn new() -> Self {
        Data { data: vec![] }
    }

    pub fn add(&mut self, log: CommitLog) {
        self.data.push(log);
    }

    pub fn print(&self) {
        println!("Pending commits to be pushed: ");

        for log in &self.data {
            println!("{}\t{}", log.commit, log.time);
        }
    }
}

impl CommitLog {
    pub fn new(commit: String, branch: String) -> Self {
        CommitLog {
            time: Utc::now(),
            commit,
            branch,
        }
    }
}
