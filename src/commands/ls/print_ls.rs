use std::{
    fs::{self, Metadata},
    io,
    path::Path,
};

use crate::{
    commands::ls::permission::{
        create_date, get_major_menor_device_number, get_symlink_target, group_user_name, is_device,
        is_executable, permissions, size_file_nlink,
    },
    utils::color::*,
};

pub fn print_dir_name(path: &Path, flag_f: bool) {
    if path.is_dir() {
        print!(
            "{BOLD}{BLUE}{}{RESET}{}  ",
            path.to_string_lossy().to_string(),
            if flag_f { "/" } else { "" }
        );
    } else {
        let is_exec = is_executable(path).unwrap_or(false);
        if path.is_symlink() {
            print_link(path);
            return;
        }
        print!(
            "{}{}{RESET}{}  ",
            if is_exec {
                GREEN.to_owned() + &BOLD.to_owned()
            } else if is_device(&path.metadata().unwrap()) {
                YELLOW.to_owned() + &BOLD.to_owned()
            } else {
                "".to_owned()
            },
            path.to_string_lossy().to_string(),
            if flag_f && is_exec { "*" } else { "" }
        )
    }
}

pub fn print_file_name(path: &Path, flag_f: bool) {
    if path.is_dir() {
        print!(
            "{BOLD}{BLUE}{}{RESET}{}  ",
            path.file_name()
                .unwrap_or(path.as_os_str())
                .to_string_lossy()
                .to_string(),
            if flag_f { "/" } else { "" }
        );
    } else {
        if path.is_symlink() {
            print_link_file_name(path)
        } else {
            let is_exec = is_executable(path).unwrap_or(false);
            print!(
                "{}{}{RESET}{}  ",
                if is_exec {
                    GREEN.to_owned() + &BOLD.to_owned()
                } else if is_device(&path.metadata().unwrap()) {
                    YELLOW.to_owned() + &BOLD.to_owned()
                } else {
                    "".to_owned()
                },
                path.file_name()
                    .unwrap_or(path.as_os_str())
                    .to_string_lossy()
                    .to_string(),
                if flag_f && is_exec { "*" } else { "" }
            )
        }
    }
}

pub fn print_link(p: &Path) {
    print!(
        "{BOLD}{SKY_DARKER}{}{RESET} -> ",
        p.to_string_lossy().to_string()
    );
    let link = get_symlink_target(p).unwrap();
    print_dir_name(Path::new(&link), false)
}

pub fn print_link_file_name(p: &Path) {
    print!(
        "{BOLD}{SKY_DARKER}{}{RESET} -> ",
        p.file_name()
            .unwrap_or(p.as_os_str())
            .to_string_lossy()
            .to_string()
    );
    let link = get_symlink_target(p).unwrap();
    print_dir_name(Path::new(&link), false)
}

pub fn print_file_info(p: &Path) -> io::Result<fs::Metadata> {
    let metadata = fs::symlink_metadata(p)?;
    let permission_file = permissions(p).unwrap_or("".to_string());
    if is_device(&metadata) {
        print_dirive_file(&permission_file, &metadata)
    } else {
        print_normal_file(&permission_file, &metadata)
    }
    Ok(metadata)
}

fn print_normal_file(per: &str, metadata: &Metadata) {
    let (user, group) = group_user_name(&metadata).unwrap_or(("".to_string(), "".to_string()));
    let (size, nlink) = size_file_nlink(&metadata);
    let creat_date = create_date(&metadata).unwrap_or("".to_string());
    print!("{per} {nlink} {group} {user} {size} {creat_date} ");
}

fn print_dirive_file(per: &str, metadata: &Metadata) {
    let (user, group) = group_user_name(&metadata).unwrap_or(("".to_string(), "".to_string()));
    let (_, nlink) = size_file_nlink(&metadata);
    let (major, minor) = get_major_menor_device_number(&metadata);
    let creat_date = create_date(&metadata).unwrap_or("".to_string());
    print!("{per} {nlink} {group} {user} {major}, {minor} {creat_date} ");
}
