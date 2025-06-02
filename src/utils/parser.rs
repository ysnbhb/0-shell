pub fn parst_input(s: String) -> Result<Vec<String>, String> {
    naive_shell_split(&s)
}

pub fn naive_shell_split(input: &str) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
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
