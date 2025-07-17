use std::{fmt::Display, fs, path::Path};

use crate::{
    commands::ls::permission::{
        get_final_component, get_symlink_target, is_broken_symlink, is_device, is_executable,
    },
    utils::{
        color::{BLUE, BOLD, CYAN, GREEN, RED, RESET, YELLOW},
        fs::is_dir,
    },
};

#[derive(Clone)]
pub struct Ls {
    pub total_bloks: u64,
    pub max_len_pr: usize,
    pub max_len_user_owner: usize,
    pub max_len_group_owner: usize,
    pub max_len_size: usize,
    pub max_len_menor: Option<usize>,
    pub max_len_mejor: Option<usize>,
    pub max_len_link: usize,
    pub flag_f: bool,
    pub files: Vec<Filee>,
}

#[derive(Ord, PartialEq, Eq, PartialOrd, Clone)]
pub struct Filee {
    pub p: String,
    pub premetion: String,
    pub size: u64,
    pub nlink: u64,
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub creat_date: String,
    pub user_owen: String,
    pub group: String,
}

impl Filee {
    pub fn new(
        p: &str,
        premetion: String,
        size: u64,
        nlink: u64,
        major: Option<u32>,
        minor: Option<u32>,
        creat_date: String,
        user_owen: String,
        group: String,
    ) -> Self {
        Self {
            p: p.to_string(),
            premetion,
            size,
            nlink,
            major,
            minor,
            creat_date,
            user_owen,
            group,
        }
    }
}

pub fn color(path: &Path) -> String {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        let file_type = metadata.file_type();

        if file_type.is_symlink() {
            if is_broken_symlink(path) {
                return format!("{}{}", BOLD, RED);
            } else {
                return format!("{}{}", BOLD, CYAN); // or SKY_DARKER
            }
        } else if file_type.is_dir() {
            return format!("{}{}", BOLD, BLUE);
        } else if is_device(&metadata) {
            return format!("{}{}", BOLD, YELLOW);
        } else if is_executable(path).unwrap_or(false) {
            return format!("{}{}", BOLD, GREEN);
        }
    }

    String::new()
}

pub fn show_file_name(p: &str, f: &mut std::fmt::Formatter<'_>, flag_f: bool) -> std::fmt::Result {
    let path = Path::new(p);
    let colore = color(path);
    let file_name = get_final_component(path).unwrap_or(path.to_string_lossy().to_string());
    write!(f, "{colore}{file_name}{RESET}")?;
    if path.is_symlink() {
        write!(f, " -> ")?;
        let linked = get_symlink_target(path).unwrap_or("".to_string());
        let linked_path = Path::new(&linked);
        let is_broklen = is_broken_symlink(path);
        if is_broklen {
            write!(f, "{colore}{linked}")?;
        } else {
            let colore = if linked_path.is_absolute() || linked_path.exists() {
                color(linked_path)
            } else {
                let resolved_path = path.join(linked_path);
                color(&resolved_path)
            };
            write!(f, "{colore}{linked}")?;
        }
        write!(f, "{RESET}")?;
    }
    if flag_f {
        if is_dir(p) {
            write!(f, "/")?;
        } else if is_executable(path).unwrap_or(false) {
            write!(f, "*")?;
        }
    }
    Ok(())
}

impl Ls {
    pub fn new() -> Self {
        Self {
            total_bloks: 0,
            max_len_pr: 0,
            max_len_user_owner: 0,
            max_len_group_owner: 0,
            max_len_size: 0,
            max_len_menor: None,
            max_len_mejor: None,
            max_len_link: 0,
            files: Vec::new(),
            flag_f: false,
        }
    }
    pub fn sort(&mut self) {
        self.files.sort_by(|a: &Filee, b: &Filee| {
            fn normalz(x: &str) -> String {
                let p = Path::new(&x);
                p.file_name()
                    .unwrap_or(p.as_os_str())
                    .to_string_lossy()
                    .to_string()
                    .trim_start_matches(".")
                    .to_lowercase()
            }
            normalz(&a.p).cmp(&normalz(&b.p))
        });
    }
    pub fn push(&mut self, f: Filee) {
        if self.max_len_pr < f.premetion.len() {
            self.max_len_pr = f.premetion.len()
        }
        if self.max_len_group_owner < f.group.len() {
            self.max_len_group_owner = f.group.len()
        }
        if let Some(major) = f.major {
            if self.max_len_mejor < Some(major.to_string().len()) {
                self.max_len_mejor = Some(major.to_string().len())
            }
        }
        if let Some(minor) = f.minor {
            if self.max_len_menor < Some(minor.to_string().len()) {
                self.max_len_menor = Some(minor.to_string().len())
            }
        }
        if self.max_len_user_owner < f.user_owen.len() {
            self.max_len_user_owner = f.user_owen.len()
        }
        if self.max_len_size < f.size.to_string().len() {
            self.max_len_size = f.size.to_string().len()
        }
        if self.max_len_link < f.nlink.to_string().len() {
            self.max_len_link = f.nlink.to_string().len()
        }
        self.files.push(f);
    }
}

impl Display for Ls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Err(e) = writeln!(f, "total {}", self.total_bloks) {
            return Err(e);
        };
        for i in self.files.clone() {
            write!(
                f,
                "{}{} ",
                i.premetion,
                " ".repeat(self.max_len_pr - i.premetion.len())
            )?;
            write!(
                f,
                "{}{} ",
                " ".repeat(self.max_len_link - i.nlink.to_string().len()),
                i.nlink,
            )?;
            write!(
                f,
                "{}{} ",
                i.user_owen,
                " ".repeat(self.max_len_user_owner - i.user_owen.len()),
            )?;
            write!(
                f,
                "{}{} ",
                i.group,
                " ".repeat(self.max_len_group_owner - i.group.len()),
            )?;

            // fix for major
            if let Some(major) = i.major {
                write!(
                    f,
                    "{}{}, ",
                    " ".repeat(self.max_len_mejor.unwrap_or(0) - major.to_string().len()),
                    major,
                )?
            } else {
                if let Some(max_len_major) = self.max_len_mejor {
                    write!(f, "{}  ", " ".repeat(max_len_major))?
                } else {
                    write!(f, "")?
                }
            }

            // fix for menor
            if let Some(minor) = i.minor {
                write!(
                    f,
                    "{}{} ",
                    " ".repeat(
                        (self.max_len_menor.unwrap_or(0).max(self.max_len_size))
                            - minor.to_string().len()
                    ),
                    minor,
                )?
            } else {
                write!(
                    f,
                    "{}{} ",
                    " ".repeat(
                        (self.max_len_menor.unwrap_or(0).max(self.max_len_size))
                            - i.size.to_string().len()
                    ),
                    i.size,
                )?
            }
            write!(f, "{} ", i.creat_date)?;
            show_file_name(&i.p, f, self.flag_f)?;
            writeln!(f)?
        }
        Ok(())
    }
}
