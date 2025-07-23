use std::{
    io::{self, Write, stdin},
    path::Path,
};

use git2::Repository;

use crate::utils::color::{BLUE, BOLD, GREEN, MAGENTA, RED, RESET, SKY_DARKER, YELLOW};

pub fn read_line() -> Option<String> {
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(0) => None,
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

pub fn git_info() -> Option<String> {
    let repo = Repository::discover(".").ok()?;

    let head = repo.head().ok()?;
    let shorthand = head.shorthand()?.to_string();

    Some(shorthand)
}

pub fn print_currant_dir(home_dir: &String, currnt_dir: String) {
    print!("{}{}➜  {}", BOLD, GREEN, SKY_DARKER);
    if *home_dir == currnt_dir {
        print!("~")
    } else {
        if let Some(file_name) = Path::new(&currnt_dir).file_name() {
            print!("{}", file_name.to_string_lossy().to_string());
        } else {
            print!("{}", currnt_dir);
        }
    }
    if let Some(branch) = git_info() {
        print!(" {}git:({RED}{branch}{BLUE})", BLUE)
    }
    print!("{} {}$ ", RESET, YELLOW);
    print!("{}", RESET);
    if let Err(e) = io::stdout().flush() {
        if e.kind() != io::ErrorKind::BrokenPipe {
            eprintln!("Failed to flush stdout: {}", e);
        }
        // If it's a BrokenPipe, just exit cleanly
        std::process::exit(0);
    }
}

pub fn print_style() {
    let r = r#"  ___                _______. __    __   _______  __       __      
 / _ \              /       ||  |  |  | |   ____||  |     |  |     
| | | |  ______    |   (----`|  |__|  | |  |__   |  |     |  |     
| | | | |______|    \   \    |   __   | |   __|  |  |     |  |     
| |_| |         .----)   |   |  |  |  | |  |____ |  `----.|  `----.
 \___/          |_______/    |__|  |__| |_______||_______||_______|
                                                                   "#;
    let mut stdout = io::stdout();
    if let Err(e) = writeln!(stdout,"{}{}{}", BOLD, MAGENTA, r) {
        if e.kind() != io::ErrorKind::BrokenPipe {
            eprintln!("Failed to flush stdout: {}", e);
        }
        std::process::exit(0); 
    }
}
