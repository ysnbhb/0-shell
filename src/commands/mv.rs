use std::{ path::Path };

use crate::utils::fs::{ move_file};

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
    sourc.iter().for_each(|f| {
        if let Err(e) = move_file(Path::new(f), Path::new(taget)) {
            println!("mv: cannot stat '{f}': {}", e.kind())
        }
    });
}
