use std::{fs, path::Path};

use crate::{commands::ls::print_ls::print_inside, utils::fs::handle_flag};

pub fn ls(paths: &[String]) {
    let res: Result<(bool, bool, bool, Vec<String>, bool), String> = handle_flag(paths);

    match res {
        Ok(ref all) => {
            if all.3.len() == 0 {
                return;
            }
            for i in all.3.clone() {
                let mut paths = Vec::new();
                if let Ok(entries) = fs::read_dir(&i) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Some(name) = entry.file_name().to_str() {
                                if name.starts_with('.') && !all.0 {
                                    continue;
                                }
                                paths.push(entry.path().as_os_str().to_string_lossy().to_string());
                            }
                        }
                    }
                    paths.sort();
                    for path in paths {
                        print_inside(Path::new(&path), all.1);
                    }
                    println!();
                }
            }
        }
        Err(e) => println!("{e}"),
    }
}
