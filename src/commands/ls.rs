use crate::utils::fs::handle_flag;

pub fn ls(paths: &[String]) {
    let res: Result<(bool, bool, bool, Vec<String>), String> = handle_flag(paths);
    match res {
        Ok(_) => println!(),
        Err(e) => println!("{e}"),
    }
}

