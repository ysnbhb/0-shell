use std::{fmt::format, path::Path};

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
        // let file1 = args[0].clone();
        // let file2 = args[1].clone();
        match fix_files( args[0].clone(),args[1].clone() ) {
            Ok(file) => {
                
            },
            Err(e)=>println!()
            
        }
    } else {
    }
}

fn fix_files(file1: String, file2: String) -> Result<(String, String), String> {
    let path1 = Path::new(&file1);
    if path1.is_dir() {
        return Err(format!("cp: omitting directory '{}'", file1));
    }
    let path2 = Path::new(&file2);
    if path2.is_file() {
        return Ok((file1, file2));
    } else {
        let file2 = file2.split("/").last().unwrap().to_string();
        Ok((file1, file2))
    }
}
