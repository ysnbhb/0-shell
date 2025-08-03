use crate::utils::fs::{copy_file, fix_files, is_dir};

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
                let res = copy_file(file.0.clone(), file.1);
                match res {
                    Ok(_) => return,
                    Err(e) => println!("cp: cannot open '{}' for reading: {}", file.0, e.kind()),
                }
            }
            Err(e) => println!("{e}"),
        }
    } else {
        let sourc = &args[..args.len() - 1];

        let to_copy = &args[args.len() - 1];
        if !is_dir(to_copy) {
            println!("cp: target '{}' is not a directory", to_copy);
            return;
        }
        for i in sourc {
            match fix_files(i.clone(), to_copy.clone()) {
                Ok(file) => {
                    let res = copy_file(file.0, file.1);
                    match res {
                        Ok(_) => continue,
                        Err(e) => println!("cp: cannot open 'todo' for reading: {}", e.kind()),
                    }
                }
                Err(e) => println!("{e}"),
            }
        }
    }
}
