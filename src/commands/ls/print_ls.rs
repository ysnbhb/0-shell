use std::path::Path;

use crate::{commands::ls::permission::is_executable, utils::color::*};

pub fn print_inside(path: &Path, flag_f: bool) {
    if path.is_dir() {
        print!(
            "{BOLD}{BLUE}{}{RESET}{}  ",
            path.as_os_str().to_string_lossy().to_string(),
            if flag_f { "/" } else { "" }
        );
    } else {
        let is_exec = is_executable(path).unwrap_or(false);
        print!(
            "{}{}{RESET}{}  ",
            if is_exec {
                GREEN.to_owned() + &BOLD.to_owned()
            } else {
                "".to_owned()
            },
            path.as_os_str().to_string_lossy().to_string(),
            if flag_f && is_exec { "*" } else { "" }
        )
    }
}