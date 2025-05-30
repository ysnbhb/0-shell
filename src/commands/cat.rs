use std::path::Path;

use crate::utils::fs::{open_file, read_file};

pub fn cat(s: &[String]) {
    for i in s {
        let file = open_file(&i);
        match file {
            Ok(f) => {
                let cont = read_file(f);
                match cont {
                    Ok(c) => print!("{c}"),
                    Err(_) => println!("cat: {i}: Is a directory"),
                }
            }
            Err(_) => {
                let path = Path::new(i);
                if path.exists() {
                    println!("cat: {i}: Permission denied");
                } else {
                    println!("cat: {i}: No such file or directory");
                    break;
                }
            }
        }
    }
}
