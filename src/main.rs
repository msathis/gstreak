use clap::{App, Arg, SubCommand};
use clap::load_yaml;
use git2::{Repository, Signature};

use crate::config::ConfigFile;
use crate::data::CommitLog;

pub mod commit;
pub mod config;
pub mod data;

fn main() {
    let yaml = load_yaml!("cli.yml");

    let matches = App::from_yaml(yaml).get_matches();
    println!("Matches {:?}", &matches.is_present("commit"));

    let path = std::env::current_dir().unwrap().display().to_string();
    let repo = match Repository::open(&path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut config = ConfigFile::new(path + "/streak.json");

    let rev = repo.head().unwrap();
    let active_branch = rev.shorthand().unwrap();

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.set_sorting(git2::Sort::TIME);
    revwalk.push_ref(format!("refs/remotes/origin/{}", active_branch).as_str());

    commit::push(active_branch, &mut config, &repo);
    commit::commit("test commit for push".to_owned(), "".to_owned(), &mut config, &repo);


    for commit in revwalk {
        let id = commit.unwrap();
        let commit = repo.find_commit(id).unwrap();
        println!("Commit {:?} : {:?}", id, commit);
    }
}
