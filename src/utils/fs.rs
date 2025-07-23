use std::fs::{File, copy, create_dir, remove_dir_all, remove_file, rename};
use std::io::Error;
use std::io::{self, Read};

use std::env;
use std::path::{Path, PathBuf};

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

pub fn is_dir(file: &str) -> bool {
    let file = Path::new(&file);
    file.is_dir()
}

pub fn is_file(file: &str) -> bool {
    let file = Path::new(&file);
    file.is_file()
}

pub fn home_dir() -> Option<String> {
    env::home_dir().map(|p| p.to_string_lossy().into_owned())
}

pub fn corrent_dir() -> Option<String> {
    match env::current_dir() {
        Ok(path) => path.to_str().map(|s| s.to_string()),
        Err(_) => env::var("PWD").ok(),
    }
}

pub fn copy_file(file1: String, file2: String) -> Result<u64, Error> {
    copy(file1, file2)
}

pub fn is_exist(file: &str) -> bool {
    let p = Path::new(&file);
    p.exists()
}

pub fn mk_dir(s: String) -> Result<(), Error> {
    create_dir(s)
}

pub fn fix_files(file1: String, file2: String) -> Result<(String, String), String> {
    if is_dir(&file1) {
        return Err(format!("cp: omitting directory '{}'", file1));
    }

    if is_dir(&file2) {
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
    if is_dir(&path) && option_r {
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
    if is_dir(path2) {
        if let Some(file_name) = file.file_name() {
            dir.join(file_name)
        } else {
            dir.to_path_buf()
        }
    } else {
        dir.to_path_buf()
    }
}
