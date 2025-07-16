pub struct Ls {
    pub max_len_pr: u32,
    pub max_len_user_owner: u32,
    pub max_len_group_owner: u32,
    pub max_len_size: u32,
    pub max_len_date: u32,
    pub max_len_menor: Option<u32>,
    pub max_len_mejor: Option<u32>,
    pub files: Vec<File>,
}

#[derive(Ord, PartialEq, Eq, PartialOrd)]
pub struct File {
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

impl File {
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
    pub fn sort(&mut self) {
        self.files.sort_by(|a, b| a.p.cmp(&b.p));
    }
}
