use std::{
    fs::{self},
    path::Path,
};

use crate::{
    commands::ls::{
        handle_flag::handle_flag,
        permission::{
            create_date, get_major_menor_device_number, group_user_name, permissions,
            size_file_nlink,
        },
        print_ls::{print_dir_name, print_file_info},
        r#struct::{Filee, Ls},
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
                let mut ls = Ls::new();
                if !is_dir(&i) {
                    let path = Path::new(&i);
                    if all.2 {
                        let _ = print_file_info(path);
                    }
                    print_dir_name(path, all.1);
                    println!();
                    continue;
                }
                if let Ok(entries) = fs::read_dir(&i) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Some(name) = entry.file_name().to_str() {
                                if name.starts_with('.') && !all.0 {
                                    continue;
                                }
                                if all.2 {
                                    let res = get_path_info(&entry.path());
                                    ls.push(res);
                                }
                                paths.push(entry.path().as_os_str().to_string_lossy().to_string());
                            }
                        }
                    }
                    if paths.is_empty() {
                        continue;
                    }
                    paths.sort();
                    ls.sort();
                    // for path in ls.files.clone() {
                    //     print_file_name(Path::new(&path.p), all.1);
                    // }
                    println!("{:?}", ls);
                    println!();
                } else {
                    println!("ls: cannot open directory '{i}': Permission denied")
                }
            }
        }
        Err(e) => println!("{e}"),
    }
}

fn get_path_info(p: &Path) -> Filee {
    let metadata = fs::symlink_metadata(p).unwrap();
    let permission_file = permissions(p).unwrap_or("".to_string());
    let (user, group) = group_user_name(&metadata).unwrap_or(("".to_string(), "".to_string()));
    let (size, nlink) = size_file_nlink(&metadata);
    let creat_date = create_date(&metadata).unwrap_or("".to_string());
    let (major, minor) = get_major_menor_device_number(&metadata);
    Filee::new(
        &p.to_string_lossy().to_string(),
        permission_file,
        size,
        nlink,
        major,
        minor,
        creat_date,
        user,
        group,
    )
}
