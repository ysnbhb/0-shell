use std::{
    io::{self, copy},
    path::Path,
};

use crate::utils::fs::{open_file, read_file};

pub fn cat(s: &[String]) {
    if s.is_empty() {
        copy_os()
    }
    for i in s {
        if i == "-" {
            copy_os();
            continue;
        }
        let file = open_file(&i);
        match file {
            Ok(f) => {
                let cont = read_file(f);
                match cont {
                    Ok(c) => print!("{c}"),
                    Err(e) => println!("cat: {i}: {}", e.kind()),
                }
            }
            Err(_) => {
                let path = Path::new(i);
                if path.exists() {
                    println!("cat: {i}: Permission denied");
                } else {
                    println!("cat: {i}: No such file or directory");
                }
            }
        }
    }
}

fn copy_os() {
    let _ = copy(&mut io::stdin(), &mut io::stdout());
}
