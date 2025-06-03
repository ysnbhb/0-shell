use std::env;
use std::fs::{File, copy};
use std::io::Error;
use std::io::{self, Read};
use std::path::Path;
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

pub fn is_exist(file :String) -> bool {
    let p = Path::new(&file);
    p.exists()
}

