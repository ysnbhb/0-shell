use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::utils::{fs::{corrent_dir, home_dir}, io::print_currant_dir};

pub fn ctrl_c() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!();
        let home_dir = home_dir().unwrap_or("".to_string());
        let current_dir = corrent_dir().unwrap_or("".to_string());
        print_currant_dir(&home_dir, current_dir);
        r.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
}
