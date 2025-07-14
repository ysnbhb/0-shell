use crate::utils::fs::is_exist;

pub fn handle_flag(strs: &[String]) -> Result<(bool, bool, bool, Vec<String>), String> {
    let mut flage_a = false;
    let mut flage_f = false;
    let mut flage_l = false;
    let mut res = Vec::new();
    let mut error = false;
    for path in strs {
        if path.starts_with("-") {
            for j in path.chars().skip(1) {
                match j {
                    'a' => flage_a = true,
                    'F' => flage_f = true,
                    'l' => flage_l = true,
                    _ => return Err(format!("ls: invalid option -- '{j}'")),
                }
            }
        } else {
            if is_exist(path.clone()) {
                res.push(path.to_string());
            } else {
                println!("ls: cannot access '{path}': No such file or directory");
                error = true
            }
        }
    }
    if res.len() == 0 && !error {
        res = vec![String::from(".")]
    }
    Ok((flage_a, flage_f, flage_l, res))
}