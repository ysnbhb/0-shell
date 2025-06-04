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
    let mut res = Vec::new();
    for i in args {
        for j in format(i) {
            res.push(j);
        }
    }
    Ok(res)
}

pub fn format(s: String) -> Vec<String> {
    if s.contains(" ") {
        return vec![s];
    }
    let mut res: Vec<String> = Vec::new();
    let mut ok = false;
    let s: Vec<_> = s.chars().collect();
    let mut i = 0;
    while i < s.len() {
        match s[i] {
            '{' => {
                if ok {
                    update_vec(&mut res, "{".to_string());
                } else {
                    let mut word = String::new();
                    let mut close = false;
                    for j in i + 1..s.len() {
                        i += 1;
                        if s[j] == '}' {
                            close = true;
                            break;
                        }
                        if s[j] == '{' {
                            break;
                        }
                        word.push(s[j]);
                    }
                    if close {
                        let new = format_input(word);
                        res = update_vec_concat_vec(res, new);
                    } else {
                        update_vec(&mut res, word);
                    }
                    ok = false;
                }
            }
            _ => update_vec(&mut res, s[i].to_string()),
        }
        i += 1
    }
    res
}

fn update_vec(arr: &mut Vec<String>, s: String) {
    if arr.is_empty() {
        arr.push(s.clone());
    } else {
        for i in 0..arr.len() {
            arr[i].push_str(&s);
        }
    }
}

fn update_vec_concat_vec(arr: Vec<String>, s: Vec<String>) -> Vec<String> {
    if arr.is_empty() {
        s
    } else {
        let mut res = Vec::new();
        for i in arr {
            for j in s.clone() {
                res.push(i.clone() + &j);
            }
        }
        res
    }
}

fn format_input(s: String) -> Vec<String> {
    if s.contains("..") {
        let parts: Vec<&str> = s.split("..").collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                if start > end {
                    return (end..=start).map(|n| n.to_string()).collect();
                }
                return (start..=end).map(|n| n.to_string()).collect();
            } else if let (Ok(start), Ok(end)) = (parts[0].parse::<char>(), parts[1].parse::<char>())
            {
                if start > end {
                    return (end..=start).map(|n| n.to_string()).collect();
                }
                return (start..=end).map(|n| n.to_string()).collect();
            }
        }
        return vec![s];
    } else {
        let res: Vec<String> = s.clone().split(",").map(String::from).collect();
        return res;
    }
}
