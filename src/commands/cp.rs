use crate::utils::fs::{copy_file, is_dir};
use std::path::{Path, PathBuf};

pub fn cp(args: &[String]) {
    if args.len() < 1 {
        println!("cp: missing file operand");
        return;
    }
    if args.len() < 2 {
        println!("cp: missing destination file operand after '{}'", args[0]);
        return;
    }
    if args.len() == 2 {
        match fix_files(args[0].clone(), args[1].clone()) {
            Ok(file) => {
                let res = copy_file(file.0, file.1);
                match res {
                    Ok(_) => return,
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{e}"),
        }
    } else {
        let sourc = &args[..args.len() - 1];
        
        let to_copy = &args[args.len() - 1];
        if !is_dir(to_copy.clone()) {
            println!("cp: target '{}' is not a directory", to_copy);
        }
        for i in sourc {
            match fix_files(i.clone(), to_copy.clone()) {
                Ok(file) => {
                    let res = copy_file(file.0, file.1);
                    match res {
                        Ok(_) => continue,
                        Err(e) => println!("{}", e),
                    }
                }
                Err(e) => println!("{e}"),
            }
        }
    }
}

fn fix_files(file1: String, file2: String) -> Result<(String, String), String> {
    if is_dir(file1.clone()) {
        return Err(format!("cp: omitting directory '{}'", file1));
    }

    if is_dir(file2.clone()) {
        let file_name = Path::new(&file1)
            .file_name()
            .ok_or_else(|| format!("cp: invalid file path '{}'", file1))?;
        let mut dest_path = PathBuf::from(file2);
        dest_path.push(file_name);

        return Ok((file1, dest_path.to_string_lossy().to_string()));
    }

    Ok((file1, file2))
}
