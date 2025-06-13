use crate::utils::fs::mk_dir;

pub fn mkdir(paths: &[String]) {
    if paths.is_empty() {
        println!("mkdir: missing operand");
        return;
    }
    for i in paths {
        if let Err(_) = mk_dir(i.clone()) {
            println!("mkdir: cannot create directory ‘{i}’: File exists")
        }
    }
}
