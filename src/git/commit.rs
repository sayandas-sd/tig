


#[derive(Debug, PartialEq)]
pub struct Commit {
    pub parent: Option<String>,
    pub message: String,
    pub author: String,
    pub time: String,
    pub hash: String,
    pub folder: String,
}   

impl Commit {
    pub fn new() -> io::Result<Self> {

    }

    pub fn get_content(self) -> String {

    }

    pub fn get_commit_from_content() -> Commit {

    }

    pub fn valid_commit(commit_data: &Vec<&str>) -> bool {

    }

    pub fn parse_data(commit_data: &Vec<str>) -> Commit {

    }
}


