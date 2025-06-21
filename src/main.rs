mod shell;
mod singals;

pub mod commands;
pub mod utils;

use shell::*;


use git2::Repository;

pub fn git_info() -> Option<(String, String)> {
    // Try opening the repository (searches upward from current dir)
    let repo = Repository::discover("./src").ok()?;

    // Get the path to the top-level repo dir (just for confirmation)
    let workdir = repo.workdir()?.to_string_lossy().into_owned();

    // Get the HEAD reference (branch name or detached state)
    let head = repo.head().ok()?;
    let shorthand = head.shorthand()?.to_string();

    Some((workdir, shorthand))
}


// fn main() {
//     shell()
// }

fn main() {
    match git_info() {
        Some((repo_path, branch)) => {
            println!("Git repo: {}", repo_path);
            println!("On branch: {}", branch);
        }
        None => println!("Not a Git repository"),
    }
}
