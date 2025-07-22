mod shell;
mod singals;

pub mod commands;
pub mod utils;

use shell::*;

use crate::singals::ctrl_c;

fn main() {
    ctrl_c();
    shell();
}
