use std::path::Path;

use crate::utils::fs::print_inside;

pub fn ls(paths: &[String]) {
    for i in paths {
        print_inside(Path::new(i));
    }
}
