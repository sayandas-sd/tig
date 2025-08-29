


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
        let mut new_content = String::new();

        for i in &new_content {

        }
    }

    pub fn get_content(self) -> String {

    }

    pub fn get_commit_from_content() -> Commit {

    }

    pub fn valid_commit(commit_data: &[&str]) -> bool {
        commit_data.len() == 6 
        && contents[0].contains("tree ")
        && contents[1].contains("parent")
        && contents[2].contains("author")
        && contents[3].contains("date_time")
        && contents[4].contains("message")
        && contents[5].contains("root_dir")

    }

    pub fn parse_data(commit_data: &Vec<str>) -> Commit {
         let parent = || -> Option<String> {
            if contents[1][7..].is_empty() {
                None
            } else {
                Some(contents[1][7..].to_string())
            }
        };
        let author: &str = &contents[2][7..];
        let date_time: &str = &contents[3][10..];
        let message: &str = &contents[4][8..];
        let root_dir: &str = &contents[5][9..];
        Commit {
            commit_hash: tree_hash.to_string(),
            message: message.to_string(),
            date_time: date_time.to_string(),
            author: author.to_string(),
            parent: parent(),
            parent_folder_name: root_dir.to_string(),
        }
    
    }
}


