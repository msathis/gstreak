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

    pub fn get_commit(&self, time: DateTime<Utc>) -> Option<&CommitLog> {
        let mut last_log = None;
        for log in &self.data {
            if log.time.le(&time) {
                last_log = Some(log);
            } else {
                break;
            }
        }
        last_log
    }
}

impl CommitLog {
    pub fn new(commit: String, branch: String, time: DateTime<Utc>) -> Self {
        CommitLog {
            time,
            commit,
            branch,
        }
    }
}
