use std::{fs, os::unix::fs::FileTypeExt, path::Path};

use crate::{
    commands::ls::{
        permission::{get_final_component, is_broken_symlink, is_device, is_executable},
        print_ls::show_file_name,
    },
    utils::color::{BLUE, BOLD, CYAN, GREEN, MAGENTA, RED, YELLOW},
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
    pub fn fmt(&self, flag_f: bool) {
        print!("{}", self);
        show_file_name(&self.p, flag_f);
    }
}

pub fn color(path: &Path) -> (String, String) {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        let file_type = metadata.file_type();

        if file_type.is_symlink() {
            if is_broken_symlink(path) {
                return (format!("{}{}", BOLD, RED), String::new());
            } else {
                return (format!("{}{}", BOLD, CYAN), String::new());
            }
        } else if file_type.is_dir() {
            return (format!("{}{}", BOLD, BLUE), String::from("/"));
        } else if is_device(&metadata) {
            return (format!("{}{}", BOLD, YELLOW), String::new());
        } else if is_executable(path).unwrap_or(false) {
            return (format!("{}{}", BOLD, GREEN), String::from("*"));
        } else if file_type.is_socket() {
            return (format!("{}{}", BOLD, MAGENTA), String::from("="));
        } else if file_type.is_fifo() {
            return (format!("{}", YELLOW), String::from("|"));
        }
    }

    (String::new(), String::new())
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
                // println!("x: {}", x);
                let p = Path::new(&x);
                let res = get_final_component(p)
                    .unwrap_or_else(|| p.to_string_lossy().to_string())
                    .trim_start_matches(".")
                    .to_string();
                res
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
