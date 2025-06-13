use std::env;
use std::fs::{File, copy, create_dir};
use std::io::Error;
use std::io::{self, Read};
use std::path::{Path , PathBuf};
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