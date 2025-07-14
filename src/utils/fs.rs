use chrono::{DateTime, Duration, Local};
use std::fs::{File, Metadata, copy, create_dir, remove_dir_all, remove_file, rename};
use std::io::Error;
use std::io::{self, Read};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::{env, fs};
use users::{get_group_by_gid, get_user_by_uid};

use crate::utils::color::{BLUE, BOLD, GREEN, RESET};

pub fn open_file(s: &str) -> io::Result<File> {
    File::open(s)
}

pub fn read_file(mut file: File) -> Result<String, Error> {
    let mut content = String::new();
    let n = file.read_to_string(&mut content);
    match n {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

pub fn is_dir(file: String) -> bool {
    let file = Path::new(&file);
    file.is_dir()
}

pub fn is_file(file: String) -> bool {
    let file = Path::new(&file);
    file.is_file()
}

pub fn home_dir() -> Option<String> {
    env::home_dir().map(|p| p.to_string_lossy().into_owned())
}

pub fn corrent_dir() -> Option<String> {
    env::current_dir()
        .ok()
        .and_then(|path| path.to_str().map(|s| s.to_string()))
}

pub fn copy_file(file1: String, file2: String) -> Result<u64, Error> {
    copy(file1, file2)
}

pub fn is_exist(file: String) -> bool {
    let p = Path::new(&file);
    p.exists()
}

pub fn mk_dir(s: String) -> Result<(), Error> {
    create_dir(s)
}

pub fn fix_files(file1: String, file2: String) -> Result<(String, String), String> {
    if is_dir(file1.clone()) {
        return Err(format!("cp: omitting directory '{}'", file1));
    }

    if is_dir(file2.clone()) {
        let file_name = Path::new(&file1)
            .file_name()
            .ok_or_else(|| format!("cp: invalid file path '{}'", file1))?;
        let mut dest_path = PathBuf::from(file2);
        dest_path.push(file_name);

        return Ok((file1, dest_path.to_string_lossy().to_string()));
    }

    Ok((file1, file2))
}

pub fn remove(path: String, option_r: bool) -> io::Result<()> {
    if is_dir(path.clone()) && option_r {
        return remove_dir_all(path);
    }
    remove_file(path)
}

pub fn move_file(from: &Path, to: PathBuf) -> Result<(), std::io::Error> {
    rename(from, to)
}

pub fn format_path(path1: &str, path2: &str) -> PathBuf {
    let file = Path::new(path1);
    let dir = Path::new(path2);
    if is_dir(path2.to_string()) {
        if let Some(file_name) = file.file_name() {
            dir.join(file_name)
        } else {
            dir.to_path_buf()
        }
    } else {
        dir.to_path_buf()
    }
}

pub fn permissions(path: &Path) -> std::io::Result<String> {
    let metadata = fs::metadata(path)?;
    let mode = metadata.mode();
    println!("{mode:03b}",);

    let file_type = metadata.file_type();
    let file_type = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_block_device() {
        'b'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_socket() {
        's'
    } else {
        '-'
    };
    let mut perms = String::new();
    perms.push(file_type);

    for i in (0..3).rev() {
        let shift = i * 3;
        let bits = (mode >> shift) & 0b111;
        println!("{bits}");
        perms.push(if (bits & 0o4) != 0 { 'r' } else { '-' });
        perms.push(if (bits & 0o2) != 0 { 'w' } else { '-' });
        perms.push(if (bits & 0o1) != 0 { 'x' } else { '-' });
    }

    Ok(perms)
}

pub fn create_date(metadata: &Metadata) -> std::io::Result<String> {
    let modified_time = metadata.modified()?;
    let date_time: DateTime<Local> = modified_time.into();
    let now = Local::now();

    let six_months = Duration::days(183);

    let formatted = if (now - date_time).abs() > six_months {
        date_time.format("%b %e  %Y").to_string()
    } else {
        date_time.format("%b %e %H:%M").to_string()
    };

    Ok(formatted)
}

pub fn group_user_name(metadata: &Metadata) -> Option<(String, String)> {
    let uid_user = metadata.uid();
    let uid_group = metadata.gid();
    let user_ownr = get_user_by_uid(uid_user)?;
    let user_group = get_group_by_gid(uid_group)?;
    Some((
        user_group.name().to_string_lossy().to_string(),
        user_ownr.name().to_string_lossy().to_string(),
    ))
}
// this function return size of file and link of it
pub fn size_file_nlink(metadata: &Metadata) -> (u64, u64) {
    (metadata.size(), metadata.nlink())
}
// this print file or dir inside of its type and if flage F is
pub fn print_inside(path: &Path, flag_f: bool) {
    if path.is_dir() {
        print!(
            "{BOLD}{BLUE}{}{RESET}{}  ",
            path.as_os_str().to_string_lossy().to_string(),
            if flag_f { "/" } else { "" }
        );
    } else {
        let is_exec = is_executable(path).unwrap_or(false);
        print!(
            "{}{}{RESET}{}  ",
            if flag_f && is_exec {
                GREEN.to_owned() + &BOLD.to_owned()
            } else {
                "".to_owned()
            },
            path.as_os_str().to_string_lossy().to_string(),
            if flag_f && is_exec { "*" } else { "" }
        )
    }
}

fn is_executable(path: &Path) -> std::io::Result<bool> {
    let metadata = path.metadata()?;
    let mode = metadata.permissions().mode();

    // Check owner, group, or others execute bit
    const OWNER_X: u32 = 0o100;
    const GROUP_X: u32 = 0o010;
    const OTHER_X: u32 = 0o001;

    Ok((mode & (OWNER_X | GROUP_X | OTHER_X)) != 0)
}

pub fn handle_flag(strs: &[String]) -> Result<(bool, bool, bool, Vec<String>,bool), String> {
    let mut flage_a = false;
    let mut flage_f = false;
    let mut flage_l = false;
    let mut res = Vec::new();
    let mut error = false;
    for path in strs {
        if path.starts_with("-") && path.len() > 1 {
            for j in path.chars().skip(1) {
                match j {
                    'a' => flage_a = true,
                    'F' => flage_f = true,
                    'l' => flage_l = true,
                    _ => return Err(format!("ls: invalid option -- '{j}'")),
                }
            }
        } else {
            if is_exist(path.clone()) {
                res.push(path.to_string());
            } else {
                println!("ls: cannot access '{path}': No such file or directory");
                error = true
            }
        }
    }
    if res.len() == 0 && !error {
        res = vec![String::from(".")]
    }
    Ok((flage_a, flage_f, flage_l, res,error))
}

pub fn get_total_blocks(dir: &Path) -> std::io::Result<u64> {
    let mut total_blocks = 0;

    for entry in dir.read_dir()? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        total_blocks += metadata.blocks();
    }

    Ok(total_blocks)
}
