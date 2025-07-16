#[derive(Debug, Clone)]
pub struct Ls {
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
