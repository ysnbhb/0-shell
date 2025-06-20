use crate::utils::fs::remove;

pub fn rm(paths: &[String]) {
    let mut remove_dir = false;
    for path in paths {
        if path.starts_with("-") {
            if
                path
                    .chars()
                    .skip(1)
                    .all(|c| c == 'r')
            {
                remove_dir = true;
            } else {
                println!(
                    "rm: invalid option -- '{}'",
                    path
                        .chars()
                        .filter(|c| *c != 'r')
                        .collect::<String>()
                );
                return;
            }
        }
    }
    let paths: Vec<&String> = paths
        .iter()
        .filter(|&word| !word.starts_with("-"))
        .collect();
    if paths.is_empty() {
        println!("rm: missing operand");
        return;
    }
    paths.iter().for_each(|path| {
        if !path.starts_with("-") {
            if
                path
                    .chars()
                    .filter(|c| *c != '/')
                    .all(|c| c == '.')
            {
                println!("rm: refusing to remove '.' or '..' directory: skipping '{path}'");
                return;
            }
            if let Err(e) = remove(path.to_string(), remove_dir) {
                println!("rm: cannot remove '{path}': {}", e.kind())
            }
        }
    });
}
