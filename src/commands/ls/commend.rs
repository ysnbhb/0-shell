use std::{fs, path::Path};

use crate::{
    commands::ls::{
        handle_flag::handle_flag,
        permission::{
            create_date, get_final_component, get_major_menor_device_number, get_total_blocks,
            group_user_name, is_executable, permissions, size_file_nlink,
        },
        r#struct::{Filee, Ls, color},
    },
    utils::{
        color::RESET,
        fs::{is_dir, is_file},
    },
};

pub fn ls(paths: &[String]) {
    let res: Result<(bool, bool, bool, Vec<String>, bool), String> = handle_flag(paths);

    match res {
        Ok((flage_a, flage_f, flage_l, mut paths, error)) => {
            if paths.len() == 0 {
                return;
            }
            if !flage_l {
                show_file_first(&mut paths, flage_f);
            }
            let paths = sort_args(paths);
            let n = paths.len();
            for (index, i) in paths.clone().iter().enumerate() {
                let path = Path::new(&i);
                if !is_dir(&i) {
                    if flage_l {
                        let file = get_path_info(path);
                        file.fmt(flage_f);
                        if n - 1 > index {
                            println!();
                        }
                    }
                    continue;
                }
                if n > 1 || error {
                    println!("{i}:")
                }
                if flage_l {
                    prin_ls_with_flagl(&i, flage_f, flage_a);
                } else {
                    prin_ls(&i, flage_f, flage_a)
                }
                if n - 1 > index {
                    println!();
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

fn prin_ls(i: &str, flag_f: bool, flag_a: bool) {
    let mut paths = Vec::new();
    let path = Path::new(i);
    if let Ok(entries) = path.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with('.') && !flag_a {
                        continue;
                    }
                    paths.push(entry.path().as_os_str().to_string_lossy().to_string());
                }
            }
        }
        if flag_a {
            paths.push(path.join(Path::new(".")).to_string_lossy().to_string());
            paths.push(path.join(Path::new("..")).to_string_lossy().to_string());
        }
        if paths.is_empty() {
            return;
        }
        paths.sort_by(|a, b| {
            fn normalz(x: &str) -> String {
                let p = Path::new(&x);
                p.file_name()
                    .unwrap_or(p.as_os_str())
                    .to_string_lossy()
                    .to_string()
                    .trim_start_matches(".")
                    .to_lowercase()
            }
            normalz(&a).cmp(&normalz(&b))
        });
        for path in paths {
            show_file_name_normal(Path::new(&path), flag_f)
        }
        println!()
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

fn show_file_normal(path: &Path, flag_f: bool) {
    let color = color(path);
    let path_name = path.to_string_lossy().to_string();
    print!("{color}{path_name}{}", RESET);
    if flag_f {
        if path.is_dir() {
            print!("/")
        } else if is_executable(path).unwrap_or(false) {
            print!("*")
        }
    }
    print!("  ");
}

pub fn show_file_first(args: &mut Vec<String>, flag_f: bool) {
    let mut ther_file = 0;
    args.iter().for_each(|path| {
        if is_file(&path) {
            ther_file += 1;
            show_file_normal(Path::new(path), flag_f);
        }
    });
    println!();
    if ther_file != args.len() && ther_file != 0 {
        println!()
    }
}

fn show_file_name_normal(path: &Path, flag_f: bool) {
    let color = color(path);
    let path_name = get_final_component(path).unwrap_or(path.to_string_lossy().to_string());
    print!("{color}{path_name}{}", RESET);
    if flag_f {
        if path.is_dir() {
            print!("/")
        } else if is_executable(path).unwrap_or(false) {
            print!("*")
        }
    }
    print!("  ")
}

fn sort_key(path: &Path) -> u8 {
    if path.is_file() {
        0
    } else if path.is_dir() {
        match fs::read_dir(path) {
            Ok(mut entries) => {
                if entries.next().is_none() {
                    1
                } else {
                    2
                }
            }
            Err(_) => 2,
        }
    } else {
        3
    }
}

pub fn sort_args(mut args: Vec<String>) -> Vec<String> {
    args.sort_by_key(|arg| {
        let path = Path::new(arg);
        sort_key(path)
    });
    args
}
