use crate::utils::fs::handle_flag;

pub fn ls(paths: &[String]) {
    let res: Result<(bool, bool, bool, Vec<String>,bool), String> = handle_flag(paths);
    match res {
        Ok(ref all) => {
            if all.3.len() == 0 {
                return;
            }
            println!("{:?}", res)
        }
        Err(e) => println!("{e}"),
    }
}
