use clap::{App, Arg, SubCommand};
use clap::load_yaml;
use git2::{Commit, Repository};

pub mod data;
pub mod commit;

fn main() {
    let yaml = load_yaml!("cli.yml");

    let matches = App::from_yaml(yaml).get_matches();
    println!("Matches {:?}", &matches.is_present("commit"));

    let path = std::env::current_dir().unwrap().display().to_string();
    let repo = match Repository::open(&path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.set_sorting(git2::Sort::TIME);
    revwalk.push_ref("FETCH_HEAD");

    let mut rp = repo.find_remote("origin").unwrap();
    rp.push();

    println!("Printing commits now");

    for commit in revwalk {
        let id = commit.unwrap();
        let commit = repo.find_commit(id).unwrap();
        println!("Commit {:?} : {:?}", id, commit);
    }
}
