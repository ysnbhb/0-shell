use crate::utils::{fs::{corrent_dir, home_dir}, io::print_currant_dir};

pub fn ctrl_c() {

    ctrlc::set_handler(move || {
        println!();
        let home_dir = home_dir().unwrap_or("".to_string());
        let current_dir = corrent_dir().unwrap_or("".to_string());
        print_currant_dir(&home_dir, current_dir);

    })
    .expect("Error setting Ctrl-C handler");
}
