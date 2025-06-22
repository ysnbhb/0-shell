use std::{ path::Path };

use crate::utils::fs::{ is_exist, move_file };

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
    for f in sourc {
        if f == taget {
            println!("mv: '{f}' and '{f}' are the same file");
        }
        if let Err(e) = move_file(Path::new(f), Path::new(taget)) {
            if is_exist(taget.clone()) {
                println!("mv: cannot stat '{f}': {}", e.kind());
            } else {
                println!("mv: target '{taget}' is not a directory");
                return;
            }
        }
    }
}
