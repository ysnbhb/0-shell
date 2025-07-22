use std::{fs, path::Path};

use crate::{
    commands::ls::{
        handle_flag::handle_flag,
        permission::{
            create_date, get_major_menor_device_number, get_total_blocks, group_user_name,
            permissions, size_file_nlink,
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
            let n = all.3.len();
            for (index, i) in all.3.clone().iter().enumerate() {
                let mut paths = Vec::new();
                let path = Path::new(&i);
                if !is_dir(&i) {
                    let path = Path::new(&i);
                    if all.2 {
                        let _ = print_file_info(path);
                    }
                    print_dir_name(path, all.1);
                    println!();
                    if n - 1 > index {
                        println!();
                    }
                    continue;
                }
                if n > 1 || all.4 {
                    println!("{i}:")
                }
                if all.2 {
                    prin_ls_with_flagl(&i, all.1, all.0);
                    if n - 1 > index {
                        println!();
                    }
                    continue;
                }
                if let Ok(entries) = path.read_dir() {
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
                    println!();
                    if n - 1 > index {
                        println!();
                    }
                } else {
                    println!("ls: cannot open directory '{i}': Permission denied")
                }
            }
        }
        Err(e) => println!("{e}"),
    }
}
fn prin_ls_with_flagl(i: &str, flag_f: bool, flag_a: bool) {
    let mut ls = Ls::new();
    ls.flag_f = flag_f;
    let path = Path::new(i);
    if let Ok(entries) = path.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with('.') && !flag_a {
                        continue;
                    }
                    let res = get_path_info(&entry.path());
                    ls.push(res);
                }
            }
        }
        if flag_a {
            let res = get_path_info(&path.join(Path::new(".")));
            ls.push(res);
            let res = get_path_info(&path.join(Path::new("..")));
            ls.push(res);
        }
        ls.total_bloks = get_total_blocks(path, flag_a).unwrap_or(0);
        ls.sort();
        print!("{}", ls);
    } else {
        println!("ls: cannot open directory '{i}': Permission denied")
    }
}

fn get_path_info(p: &Path) -> Filee {
    let metadata = fs::symlink_metadata(p).unwrap();
    let permission_file = permissions(p).unwrap_or("".to_string());
    let (group, user) = group_user_name(&metadata).unwrap_or(("".to_string(), "".to_string()));
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

