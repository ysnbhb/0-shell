use std::fmt::Debug;

#[derive(Clone)]
pub struct Ls {
    pub total_bloks: u64,
    pub max_len_pr: usize,
    pub max_len_user_owner: usize,
    pub max_len_group_owner: usize,
    pub max_len_size: usize,
    pub max_len_date: usize,
    pub max_len_menor: Option<usize>,
    pub max_len_mejor: Option<usize>,
    pub max_len_link: usize,
    pub files: Vec<Filee>,
}

#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Clone)]
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

impl Ls {
    pub fn new() -> Self {
        Self {
            total_bloks: 0,
            max_len_pr: 0,
            max_len_user_owner: 0,
            max_len_group_owner: 0,
            max_len_size: 0,
            max_len_date: 0,
            max_len_menor: None,
            max_len_mejor: None,
            max_len_link: 0,
            files: Vec::new(),
        }
    }
    pub fn sort(&mut self) {
        self.files.sort_by(|a, b| a.p.cmp(&b.p));
    }
    pub fn push(&mut self, f: Filee) {
        if self.max_len_pr < f.premetion.len() {
            self.max_len_pr = f.premetion.len()
        }
        if self.max_len_date < f.creat_date.len() {
            self.max_len_date = f.creat_date.len()
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

impl Debug for Ls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Err(e) = writeln!(f, "total {}", self.total_bloks) {
            return Err(e);
        };
        for i in self.files.clone() {
            if let Err(e) = write!(
                f,
                "{}{} ",
                i.premetion,
                " ".repeat(self.max_len_pr - i.premetion.len())
            ) {
                return Err(e);
            };
            if let Err(e) = write!(
                f,
                "{}{} ",
                " ".repeat(self.max_len_link - i.nlink.to_string().len()),
                i.nlink,
            ) {
                return Err(e);
            };
            if let Err(e) = write!(
                f,
                "{}{} ",
                i.user_owen,
                " ".repeat(self.max_len_user_owner - i.user_owen.len()),
            ) {
                return Err(e);
            };
            if let Err(e) = write!(
                f,
                "{}{} ",
                i.group,
                " ".repeat(self.max_len_group_owner - i.group.len()),
            ) {
                return Err(e);
            };
            if let Some(major) = i.major {
                if let Err(e) = write!(
                    f,
                    "{}{}, ",
                    " ".repeat(self.max_len_mejor.unwrap_or(0) - major.to_string().len()),
                    major,
                ) {
                    return Err(e);
                };
            } else {
                if let Some(max_len_major) = self.max_len_mejor {
                    if let Err(e) = write!(f, "{} ", " ".repeat(max_len_major)) {
                        return Err(e);
                    };
                } else {
                    if let Err(e) = write!(f, "") {
                        return Err(e);
                    };
                }
            }
        }
        Ok(())
    }
}
