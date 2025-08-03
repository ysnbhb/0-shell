use std::io::{self, Write};

pub fn parst_input(s: String, home_dir: &str) -> Result<Vec<String>, String> {
    let mut input = s.trim().to_string();
    if input.is_empty() {
        return Ok(vec![]);
    }

    loop {
        match naive_shell_split(&input, home_dir) {
            Ok(mut args) => {
                if let Some(pos) = args.iter().position(|arg| arg == "--") {
                    args.remove(pos);
                }
                return Ok(args);
            }
            Err(e) => {
                print!("> ");
                io::stdout().flush().unwrap();

                if let Some(line) = read_line() {
                    input.push_str(&format!(
                        "{}{}",
                        if e == "Trailing backslash in input" {
                            " "
                        } else {
                            "\n"
                        },
                        line.trim_end()
                    ));
                } else {
                    return Err(e);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum QuoteState {
    None,
    Double,
    Single,
    Backtick,
}

use QuoteState::*;

use crate::utils::io::read_line;

pub fn naive_shell_split(input: &str, home_dir: &str) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut quote_state = None;
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
                        None => {
                            quote_state = Double;
                        }
                        Double => {
                            quote_state = None;
                        }
                        Single | Backtick => {
                            current.push('"');
                        }
                    }
                }
            }
            '\'' => {
                if is_escaped && quote_state != Single {
                    current.push('\'');
                    is_escaped = false;
                } else {
                    match quote_state {
                        None => {
                            quote_state = Single;
                        }
                        Single => {
                            quote_state = None;
                        }
                        Double | Backtick => {
                            current.push('\'');
                        }
                    }
                }
            }
            ' ' | '\t' => {
                if quote_state != None {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        args.push(expand_tilde(&current, &home_dir));
                        current.clear();
                    }

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
                if quote_state == Single {
                    current.push('\\');
                } else if is_escaped {
                    if let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            'n' => {
                                chars.next();
                                current.push('\n');
                            }
                            't' => {
                                chars.next();
                                current.push('\t');
                            }
                            'r' => {
                                chars.next();
                                current.push('\r');
                            }
                            '\\' => {
                                chars.next();
                                current.push('\\');
                            }
                            _ => {
                                current.push('\\');
                            }
                        }
                    } else {
                        current.push('\\');
                    }
                    is_escaped = false;
                } else {
                    is_escaped = true;
                }
            }
            '`' => {
                if is_escaped && quote_state != Backtick {
                    current.push('`');
                    is_escaped = false;
                } else {
                    match quote_state {
                        None => {
                            quote_state = Backtick;
                        }
                        Backtick => {
                            quote_state = None;
                        }
                        Double | Single => {
                            current.push('`');
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
        Double => return Err("Unclosed double quote in input".to_string()),
        Single => return Err("Unclosed single quote in input".to_string()),
        Backtick => return Err("Unclosed back tick quote in input".to_string()),
        None => {}
    }

    // Check for trailing escape
    if is_escaped {
        return Err("Trailing backslash in input".to_string());
    }
    if !current.is_empty() {
        args.push(expand_tilde(&current, &home_dir));
    }
    // println!("{:?}" , args);

    // Process the final result with expansions

    Ok(args)
}

fn expand_tilde(input: &str, home_dir: &str) -> String {
    if input.starts_with("~/") || input == "~" {
        input.replacen("~", home_dir, 1)
    } else {
        input.to_string()
    }
}