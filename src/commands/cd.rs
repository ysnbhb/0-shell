use std::{env, path::Path};

pub fn cd(s: &str) {
    let res = env::set_current_dir(s);
    if res.is_err() {
        // println!("{:?}" , res.err())
        let path = Path::new(s);
        if path.is_file() {
            println!("cd: not a directory: {:?}", s)
        } else {
            println!("no such file or directory: {:?}", s)
        }
    }
}
