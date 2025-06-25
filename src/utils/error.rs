use crate::utils::color::{BOLD, RED, RESET};

pub fn print_error(s: &str) {
    println!(" {RED}{BOLD}error:{RESET} {s}")
}
