pub fn parst_input(s: String) -> Result<Vec<String>, String> {
    naive_shell_split(&s)
}

pub fn naive_shell_split(input: &str) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut is_back = false;
    for c in input.chars() {
        match c {
            '"' => {
                if is_back {
                    current.push('"');
                    is_back = false;
                    continue;
                }
                in_quotes = !in_quotes
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            '\\' => {
                if is_back {
                    current.push('\\');
                }
                is_back = !is_back
            }
            _ => {
                if is_back {
                    current.push('\\');
                }
                current.push(c)
            }
        }
    }

    if in_quotes {
        return Err("Unclosed quote in input".to_string());
    }

    if !current.is_empty() {
        args.push(current);
    }
    Ok(args)
}
