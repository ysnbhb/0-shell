use core::fmt;
use std::{fmt::Display, path::Path};

use crate::{
    commands::ls::{
        permission::{get_final_component, get_symlink_target, is_broken_symlink},
        r#struct::{Filee, Ls, color},
    },
    utils::color::RESET,
};

impl fmt::Display for Filee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.premetion)?;
        write!(f, "{} ", self.user_owen)?;
        write!(f, "{} ", self.group)?;
        write!(f, "{} ", self.nlink)?;
        if let Some(major) = self.major {
            write!(f, "{}, ", major)?;
        } else {
            write!(f, "")?;
        }
        write!(f, "{} ", self.minor.unwrap_or(self.size as u32))?;
        write!(f, "{}", self.creat_date)
    }
}

pub fn show_file_name_of_display(
    p: &str,
    f: &mut std::fmt::Formatter<'_>,
    flag_f: bool,
) -> std::fmt::Result {
    let path = Path::new(p);
    let (colore, types) = color(path);
    let file_name = get_final_component(path).unwrap_or(path.to_string_lossy().to_string());
    write!(f, "{colore}{file_name}{RESET}")?;
    if flag_f {
        write!(f, "{}", types)?;
    }
    if path.is_symlink() {
        write!(f, " -> ")?;
        let linked = get_symlink_target(path).unwrap_or("".to_string());
        let linked_path = Path::new(&linked);
        let is_broklen = is_broken_symlink(path);
        if is_broklen {
            write!(f, "{colore}{linked}")?;
        } else {
            let (colore, types) = if linked_path.is_absolute() || linked_path.exists() {
                color(linked_path)
            } else {
                let resolved_path = path.join(linked_path);
                color(&resolved_path)
            };
            write!(f, "{colore}{linked}")?;
            if flag_f {
                write!(f, "{RESET}")?;
                write!(f, "{types}")?;
            }
        }
        
    }
    Ok(())
}

pub fn show_file_name(p: &str, flag_f: bool) {
    let path = Path::new(p);
    let (colore, types) = color(path);
    let file_name = get_final_component(path).unwrap_or(path.to_string_lossy().to_string());
    print!("{colore}{file_name}{RESET}");
    if flag_f {
        print!("{types}");
    }
    if path.is_symlink() {
        print!(" -> ");
        let linked = get_symlink_target(path).unwrap_or("".to_string());
        let linked_path = Path::new(&linked);
        let is_broklen = is_broken_symlink(path);
        if is_broklen {
            print!("{colore}{linked}");
        } else {
            let (colore, types) = if linked_path.is_absolute() || linked_path.exists() {
                color(linked_path)
            } else {
                let resolved_path = path.join(linked_path);
                color(&resolved_path)
            };
            print!("{colore}{linked}");
            if flag_f {
                print!("{RESET}");
                print!("{types}");
            }
        }
        print!("{RESET}");
    }

    println!()
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
            show_file_name_of_display(&i.p, f, self.flag_f)?;
            writeln!(f, "{}", RESET)?
        }
        Ok(())
    }
}
