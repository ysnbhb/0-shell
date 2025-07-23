use std::env;

use crate::utils::fs::corrent_dir;

pub fn cd(s: &str) {
    let current = corrent_dir().unwrap_or("".to_string());
    let res = env::set_current_dir(s);
    if let Err(e) = res {
        println!("ls : {}: {:?}", e.kind(), s)
    } else {
        unsafe {
            env::set_var("OLDPWD", current);
        }
        let current = corrent_dir().unwrap_or("".to_string());
        unsafe {
            env::set_var("PWD", current);
        }
    }
}
