use git2::{Commit, Repository};

fn main() {
    let repo = match Repository::open("/Users/16736/Desktop/Myntra/virian") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.set_sorting(git2::Sort::TIME);

    revwalk.push_head();

    println!("Printing commits now");

    for commit in revwalk {
        let id = commit.unwrap();
        let commit = repo.find_commit(id).unwrap();
        println!("Commit {:?} : {:?}", id, commit);
    }

}
