use git2::Repository;

use crate::config::ConfigFile;
use crate::data;
use crate::data::CommitLog;

pub fn commit(message: String, date: String, config: &mut ConfigFile, repo: &Repository) {
    let sig = repo.signature().unwrap();
    let mut index = repo.index().unwrap();

    let head = index.write_tree().unwrap();
    let last_commit = repo.find_commit(head).unwrap();
    let tree = repo.find_tree(head).unwrap();

    let commit_id = repo.commit(None, &sig, &sig, &message, &tree, &[&last_commit]).unwrap();
    config.add_log(CommitLog::new(commit_id.to_string()));
}
