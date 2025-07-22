use crate::match_command;
use std::env::{self};

pub fn parst_input(s: String, home_dir: String) -> Result<Vec<String>, String> {
    naive_shell_split(&s, home_dir)
}

#[derive(Debug, Clone, PartialEq)]
enum QuoteState {
    None,
    Double,
    Single,
}

pub fn naive_shell_split(input: &str, home_dir: String) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut quote_state = QuoteState::None;
    let mut is_escaped = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if is_escaped {
                    current.push('"');
                    is_escaped = false;
                } else {
                    match quote_state {
                        QuoteState::None => {
                            quote_state = QuoteState::Double;
                        }
                        QuoteState::Double => {
                            args.push(current.clone());
                            quote_state = QuoteState::None;
                        }
                        QuoteState::Single => {
                            current.push('"');
                        }
                    }
                }
            }
            '\'' => {
                if is_escaped && quote_state != QuoteState::Single {
                    current.push('\'');
                    is_escaped = false;
                } else {
                    match quote_state {
                        QuoteState::None => {
                            quote_state = QuoteState::Single;
                        }
                        QuoteState::Single => {
                            quote_state = QuoteState::None;
                        }
                        QuoteState::Double => {
                            current.push('\'');
                        }
                    }
                }
            }
            ' ' | '\t' => {
                if quote_state != QuoteState::None {
                    current.push(c);
                } else {
                    args.push(expand_tilde(&current, &home_dir));
                    current.clear();

                    // Skip multiple whitespace
                    while let Some(&next_c) = chars.peek() {
                        if next_c == ' ' || next_c == '\t' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                is_escaped = false;
            }
            '\\' => {
                if quote_state == QuoteState::Single {
                    // In single quotes, backslash is literal
                    current.push('\\');
                } else if is_escaped {
                    // We have \\, check what comes next
                    if let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            'n' => {
                                chars.next(); // consume the 'n'
                                current.push('\n');
                            }
                            't' => {
                                chars.next(); // consume the 't'
                                current.push('\t');
                            }
                            'r' => {
                                chars.next(); // consume the 'r'
                                current.push('\r');
                            }
                            '\\' => {
                                chars.next(); // consume the second backslash
                                current.push('\\');
                            }
                            _ => {
                                // Unknown escape after \\, just output literal backslash and the character
                                current.push('\\');
                            }
                        }
                    } else {
                        // \\ at end of string
                        current.push('\\');
                    }
                    is_escaped = false;
                } else {
                    is_escaped = true;
                }
            }
            ';' => {
                if quote_state != QuoteState::None {
                    current.push(c);
                } else {
                    if is_escaped {
                        current.push(';');
                        is_escaped = false;
                    } else {
                        args.push(expand_tilde(&current, &home_dir));
                        current.clear();

                        if !args.is_empty() {
                            if args[0] == "exit" {
                                return Ok(args);
                            }
                            // Here you would call your match_command function
                            // match_command(&args, home_dir.clone());
                            match_command(&args, &home_dir);
                        } else {
                            return Err("syntax error near unexpected token `;'".to_string());
                        }
                    }
                }
            }
            _ => {
                if is_escaped {
                    current.push(c);
                    is_escaped = false;
                } else {
                    current.push(c);
                }
            }
        }
    }

    // Check for unclosed quotes
    match quote_state {
        QuoteState::Double => return Err("Unclosed double quote in input".to_string()),
        QuoteState::Single => return Err("Unclosed single quote in input".to_string()),
        QuoteState::None => {}
    }

    // Check for trailing escape
    if is_escaped {
        return Err("Trailing backslash in input".to_string());
    }
    // println!("{:?}" , args);
    args.push(expand_tilde(&current, &home_dir));

    // Process the final result with expansions
    let mut result = Vec::new();
    for arg in args {
        for expanded in format(arg) {
            if expanded.contains('$') {
                let expanded_var = expand_env_vars(expanded);
                if !expanded_var.is_empty() {
                    result.push(expanded_var);
                }
            } else {
                result.push(expanded);
            }
        }
    }

    Ok(result)
}

fn expand_tilde(input: &str, home_dir: &str) -> String {
    if input.starts_with("~/") || input == "~" {
        input.replacen("~", home_dir, 1)
    } else {
        input.to_string()
    }
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

pub fn expand_env_vars(s: String) -> String {
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
