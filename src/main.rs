mod shell;
mod signals;

pub mod commands;
pub mod utils;

use shell::*;

use crate::signals::ctrl_c;

fn main() {
    ctrl_c();
    shell();
}
