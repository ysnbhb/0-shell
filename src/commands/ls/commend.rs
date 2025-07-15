use std::{fs, io, path::Path};

use crate::{
    commands::ls::{
        handle_flag::handle_flag,
        permission::{create_date, group_user_name, permissions, size_file_nlink},
        print_ls::print_inside,
    },
    utils::fs::is_dir,
};

pub fn ls(paths: &[String]) {
    let res: Result<(bool, bool, bool, Vec<String>, bool), String> = handle_flag(paths);

    match res {
        Ok(ref all) => {
            if all.3.len() == 0 {
                return;
            }
            for i in all.3.clone() {
                let mut paths = Vec::new();
                if !is_dir(&i) {
                    let path = Path::new(&i);
                    if all.2 {
                        let _ = print_file_info(path);
                    }
                    print_inside(path, all.1);
                    if all.2 {
                        println!();
                    }
                    continue;
                }
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
                    if paths.is_empty() {
                        continue;
                    }
                    paths.sort();
                    for path in paths {
                        print_inside(Path::new(&path), all.1);
                    }
                    println!();
                } else {
                    println!("ls: cannot open directory '{i}': Permission denied")
                }
            }
        }
        Err(e) => println!("{e}"),
    }
}

fn print_file_info(p: &Path) -> io::Result<fs::Metadata> {
    let metadata = p.metadata()?;
    let permission_file = permissions(p).unwrap_or("".to_string());
    let (user, group) = group_user_name(&metadata).unwrap_or(("".to_string(), "".to_string()));
    let (size, nlink) = size_file_nlink(&metadata);
    let creat_date = create_date(&metadata).unwrap_or("".to_string());
    print!("{permission_file} {nlink} {group} {user} {size} {creat_date} ");
    Ok(metadata)
}
