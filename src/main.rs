use clap::App;
use clap::load_yaml;
use git2::Repository;
use anyhow::{Error, Result};

use crate::commit::Committer;
use crate::config::ConfigFile;

pub mod commit;
pub mod config;
pub mod data;

fn main() -> Result<(), Error>{
    let yaml = load_yaml!("cli.yml");

    let matches = App::from_yaml(yaml)
        .get_matches();

    let path = std::env::current_dir().unwrap().display().to_string();
    let repo = match Repository::open(&path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut config = ConfigFile::new(path + "/streak.json");
    let rev = repo.head().unwrap();
    let active_branch = rev.shorthand().unwrap();
    let mut committer = Committer::new(&mut config, &repo);

    if matches.is_present("commit") {
        let matches = matches.subcommand_matches("commit").unwrap();
        committer.commit(matches.value_of("message").unwrap().to_string(),
                         matches.value_of("time"))?;
    } else if matches.is_present("list") {
        committer.print_logs();
    } else if matches.is_present("check") {
        committer.print_next_commit();
    } else if matches.is_present("push") {
        committer.push(active_branch)?;
    }
    Ok(())
}
