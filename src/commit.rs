use std::env;

use git2::{PushOptions, Repository};
use git2_credentials::CredentialHandler;

use crate::config::ConfigFile;
use crate::data::CommitLog;

pub struct Committer<'a> {
    options: PushOptions<'a>,
    repo: &'a Repository,
    config: &'a mut ConfigFile,
}

impl<'a> Committer<'a> {
    pub fn new(config: &'a mut ConfigFile, repo: &'a Repository) -> Self {
        Committer {
            repo,
            config,
            options: Self::get_push_options(),
        }
    }

    pub fn commit(&mut self, message: String, date: String) {
        let sig = self.repo.signature().unwrap();

        let tree_id = self.repo.index().unwrap().write_tree().unwrap();
        let tree = self.repo.find_tree(tree_id).unwrap();

        let parent_id = self.repo.head().ok().and_then(|h| h.target()).unwrap();
        let parent = self.repo.find_commit(parent_id).unwrap();

        let commit_id = self.repo.commit(Some("HEAD"), &sig,
                                         &sig, &message, &tree, &[&parent]).unwrap();

        self.config.add_log(CommitLog::new(commit_id.to_string()));
    }

    pub fn push(&mut self, branch: &str) {
        let mut remote = self.repo.find_remote("origin").unwrap();
        let origin = format!("refs/heads/{}", branch);


        match remote.push(&[&origin], Some(&mut self.options)) {
            Err(e) => println!("Push to remote failed {}", e),
            Ok(_) => println!("Push successful")
        }
    }

    pub fn get_push_options() -> PushOptions<'a> {
        let mut opts = git2::PushOptions::new();
        let mut cb = git2::RemoteCallbacks::new();
        let git_config = git2::Config::open_default().unwrap();
        let mut ch = CredentialHandler::new(git_config);
        cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));
        opts.remote_callbacks(cb);

        opts
    }
}



