use std::path::Path;

use crate::utils::fs::{format_path, is_exist, move_file};

pub fn mv(paths: &[String]) {
    if paths.is_empty() {
        println!("mv: missing file operand");
        return;
    }
    if paths.len() == 1 {
        println!("mv: missing destination file operand after '{}'", paths[0]);
        return;
    }
    let taget = paths.last().unwrap();
    let sourc = &paths[0..paths.len() - 1];
    move_files(sourc, taget);
}

fn move_files(files: &[String], dir: &String) {
    for f in files {
        if f == dir {
            println!("mv: '{f}' and '{f}' are the same file");
            continue;
        }
        let path2 = format_path(f, dir);
        if let Err(e) = move_file(Path::new(f), path2) {
            if is_exist(dir) {
                println!("mv: cannot stat '{f}' to {dir}: {}", e.kind());
            } else {
                println!("mv: target '{dir}' is not a directory");
                return;
            }
        }
    }
}
