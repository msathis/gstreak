use clap::App;
use clap::load_yaml;
use git2::Repository;

use crate::commit::Committer;
use crate::config::ConfigFile;

pub mod commit;
pub mod config;
pub mod data;

fn main() {
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
        committer.commit(matches.value_of("message").unwrap().to_string(),
                         matches.value_of("date"));
    } else if matches.is_present("list") {
        committer.print_logs();
    } else if matches.is_present("push") {
        committer.push(active_branch);
    }

    //let mut revwalk = repo.revwalk().unwrap();
    //revwalk.set_sorting(git2::Sort::TIME);
    //revwalk.push_ref(format!("refs/remotes/origin/{}", active_branch).as_str());


    //committer.push(active_branch);
    //committer.commit("Add method to list unpushed logs".to_owned(), "".to_owned());


//    for commit in revwalk {
//        let id = commit.unwrap();
//        let commit = repo.find_commit(id).unwrap();
//        println!("Commit {:?} : {:?}", id, commit);
//    }
}
