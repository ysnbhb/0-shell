use crate::utils::fs::remove;

pub fn rm(paths: &[String]) {
    let mut flag_r = false;
    let mut res: Vec<&String> = Vec::new();
    for path in paths {
        if path.starts_with("-") {
            if handul_flag(path.clone()) {
                flag_r = true;
            } else {
                return;
            }
        } else {
            res.push(path);
        }
    }
    if paths.is_empty() {
        println!("rm: missing operand");
        return;
    }
    paths.iter().for_each(|path| {
        if path.chars().filter(|c| *c != '/').all(|c| c == '.') {
            println!("rm: refusing to remove '.' or '..' directory: skipping '{path}'");
        } else if let Err(e) = remove(path.to_string(), flag_r) {
            println!("rm: cannot remove '{path}': {}", e.kind())
        }
    });
}

fn handul_flag(s: String) -> bool {
    for i in s.chars().skip(1) {
        if i != 'r' {
            println!("rm: invalid option -- '{i}'");
            return false;
        }
    }
    true
}
