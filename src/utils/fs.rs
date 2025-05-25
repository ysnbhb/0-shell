use std::io::Error;
use std::fs::File;
use std::io::{self, Read};
pub fn open_file(s: &str) -> io::Result<File> {
    File::open(s)
}

pub fn read_file(mut file: File) ->Result<String , Error> {
    let mut content = String::new();
    let n = file.read_to_string(&mut content);
    match n {
        Ok(_)=> Ok(content),
        Err(e) => Err(e)
    }
}
