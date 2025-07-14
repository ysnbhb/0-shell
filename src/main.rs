mod shell;
mod singals;

pub mod commands;
pub mod utils;


use std::path::Path;

use shell::*;

use crate::commands::ls::permission::get_major_device_number;


fn main() {
    println!("{:?}",get_major_device_number(Path::new("/dev/vfio")))
}
