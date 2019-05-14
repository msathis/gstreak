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

    pub fn get_commit(&self, time: DateTime<Utc>) -> Option<String> {
        let mut last_log = None;
        for log in &self.data {
            if log.time.le(&time) {
                last_log = Some(log.commit.clone());
            } else {
                break;
            }
        }
        last_log
    }

    pub fn has_logs(&self) -> bool {
        self.data.len() > 0
    }

    pub fn clear_logs(&mut self, commit: &str) {
        let mut i = 0;
        for log in &self.data {
            i += 1;
            if log.commit == commit {
                break;
            }
        }
        self.data.drain(0..i);
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

    pub fn get_commit(&self) -> &String {
        &self.commit
    }

    pub fn get_time(&self) -> &DateTime<Utc> {
        &self.time
    }
}
