use crate::match_command;
use std::env::{self};

pub fn parst_input(s: String, home_dir: String) -> Result<Vec<String>, String> {
    naive_shell_split(&s, home_dir)
}

pub fn naive_shell_split(input: &str, home_dir: String) -> Result<Vec<String>, String> {
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
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone().replacen("~", &home_dir, 1));
                    current.clear();
                }
            }
            '\t' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone().replacen("~", &home_dir, 1));
                    current.clear();
                }
            }
            '\\' => {
                if is_back {
                    current.push('\\');
                }
                is_back = !is_back;
            }
            ';' => {
                if in_quotes {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        args.push(current.clone());
                        current.clear();
                    }
                    if !args.is_empty() {
                        if args[0] == "exit" {
                            return Ok(args);
                        }
                        match_command(&args, home_dir.clone());
                        args.clear();
                    } else {
                        return Err("syntax error near unexpected token `;'".to_string());
                    }
                }
            }
            _ => {
                if is_back {
                    current.push('\\');
                }
                current.push(c);
            }
        }
    }

    if in_quotes {
        return Err("Unclosed quote in input".to_string());
    }

    if !current.is_empty() {
        args.push(current.replacen("~", &home_dir, 1));
    }
    let mut res = Vec::new();
    for i in args {
        for j in format(i) {
            if j.contains("$") {
                let word = env(j);
                if word.is_empty() {
                    continue;
                } else {
                    res.push(word);
                }
            } else {
                res.push(j);
            }
        }
    }
    Ok(res)
}

pub fn format(s: String) -> Vec<String> {
    if s.contains(" ") {
        return vec![s];
    }
    let mut res: Vec<String> = Vec::new();
    let s: Vec<_> = s.chars().collect();
    let mut i = 0;
    while i < s.len() {
        match s[i] {
            '{' => {
                let next = s.get(i + 1).unwrap_or(&'\0');
                if *next == '{' {
                    update_vec(&mut res, "{".to_string());
                } else {
                    let mut word = String::new();
                    let mut close = 0;
                    for j in i..s.len() {
                        i += 1;
                        if s[j] == '}' {
                            close -= 1;
                        } else if s[j] == '{' {
                            close += 1;
                        }
                        word.push(s[j]);
                        if close == 0 {
                            break;
                        }
                    }
                    i -= 1;
                    if close == 0 {
                        if !word.contains(",") && !word.contains("..") {
                            update_vec(&mut res, word);
                        } else {
                            let new = format_input(word);
                            res = update_vec_concat_vec(res, new);
                        }
                    } else {
                        update_vec(&mut res, word);
                    }
                }
            }
            _ => update_vec(&mut res, s[i].to_string()),
        }
        i += 1;
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
        let copy = s
            .chars()
            .filter(|c| (*c != '{' && *c != '}'))
            .collect::<String>();
        let parts: Vec<&str> = copy.split("..").collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                if start > end {
                    return (end..=start).map(|n| n.to_string()).collect();
                }
                return (start..=end).map(|n| n.to_string()).collect();
            } else if let (Ok(start), Ok(end)) =
                (parts[0].parse::<char>(), parts[1].parse::<char>())
            {
                if start > end {
                    return (end..=start).map(|n| n.to_string()).collect();
                }
                return (start..=end).map(|n| n.to_string()).collect();
            }
        }
        return vec![s];
    } else {
        let filter: String = s
            .chars()
            .filter(|&c| c != '{' && c != '}')
            .collect::<String>();
        let res: Vec<String> = filter.clone().split(",").map(String::from).collect();
        return res;
    }
}

pub fn env(s: String) -> String {
    if s == "$" {
        return s;
    }
    let mut is_doller = false;
    let mut word = String::new();
    let mut res = String::new();
    for i in s.chars() {
        if i == '$' {
            if is_doller {
                if word.is_empty() {
                    word = "$".to_string();
                }
                res.push_str(&env::var(word.clone()).unwrap_or("".to_string()));
                word.clear();
            }
            is_doller = !is_doller;
        } else if i == '/' {
            if is_doller {
                res.push_str(&env::var(word.clone()).unwrap_or("".to_string()));
                res.push('/');
                word.clear();
                is_doller = false;
            } else {
                word.push(i);
            }
        } else {
            word.push(i);
        }
    }
    if !word.is_empty() {
        if is_doller {
            res.push_str(&env::var(word.clone()).unwrap_or("".to_string()));
        } else {
            res.push_str(&word);
        }
    } else if is_doller {
        res.push('$');
    }
    res
}
