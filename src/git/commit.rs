
use chorono::Utc;

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
    pub fn new(
        message: &str,
        repo_tree_hash: String,
        pr_folder: String
    ) -> io::Result<Self> {
        let date_time: String = Utc::now().to_string();
        let username: String = whoami::realname();
        let parent: Option<yaml_layouts::Commit> = ConfigLayout::get_last_commit();
        let commit_hash: String = root_repository_tree_hash;

         match parent {
            Some(parent) => {
                let parent: Option<String> = Some(parent.hash);
                Ok(Commit {
                    date_time,
                    message: commit_message.to_string(),
                    author: username,
                    commit_hash,
                    parent,
                    parent_folder_name,
                })
            }
            None => Ok(Commit {
                date_time,
                message: commit_message.to_string(),
                author: username,
                commit_hash,
                parent: Option::None,
                parent_folder_name,
            }),
        }
    }

    pub fn get_content(self) -> String {
         let mut content: String = String::new();
        let date_str: &String = &self.date_time.to_string();
        content.push_str("tree ");
        content.push_str(&self.commit_hash);
        content.push('\n');
        content.push_str("parent ");
        match self.parent {
            Some(parent_hash) => {
                content.push_str(&parent_hash);
                content.push('\n')
            }
            None => content.push('\n'),
        }
        content.push_str("author ");
        content.push_str(&self.author);
        content.push('\n');
        content.push_str("date_time ");
        content.push_str(&date_str);
        content.push('\n');
        content.push_str("message ");
        content.push_str(&self.message);
        content.push('\n');
        content.push_str("root_dir ");
        content.push_str(&self.parent_folder_name);
        content

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


