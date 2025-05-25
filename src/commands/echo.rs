pub fn echo(strs: &[String]) {
    let s = strs.join(" ");
    let mut res = String::new();
    let mut word = String::new();
    for i in s.chars() {
        if i == '\\' {
            word.push(i);
            if word == "\\\\" {
                res.push('\\');
                word.clear();
            }
        } else {
            word.push(i);
            if word == "\\n" {
                res.push('\n');
            } else if word == "\\a" {
                res.push(7 as char);
            } else if word == "\\b" {
                res.push(8 as char);
            } else if word == "\\t" {
                res.push('\t');
            } else if word == "\\v" {
                res.push(13 as char);
            } else if word == "\\f" {
                res.push(14 as char);
            } else if word == "\\r" {
                res.push('\r');
            } else {
                res.push_str(&word);
            }
            word.clear();
        }
    }
    if !word.is_empty() {
        res.push_str(&word);
    }
    println!("{res}");
}
