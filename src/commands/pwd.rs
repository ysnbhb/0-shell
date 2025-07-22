use crate::utils::fs::corrent_dir;

pub fn pwd() {
    match corrent_dir() {
        Some(path) => println!("{}", path),
        None => println!("filed to get corrent dir"),
    }
}
