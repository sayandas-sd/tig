

#[derive(Debug, PartialEq, Clone)]
pub struct Tree {
    pub content: String,
    pub content_size: i32,
    pub data: Vec<InitTree>
}

#[derive(Debug, PartialEq, Clone)]
pub struct InitTree {
    pub name: String,
    pub object: String,
    pub hash: String
}

impl Tree {
    pub fn create_tree() {

    }

    pub fn get_tree_content() {

    }

    pub fn get_content_from_tree() {

    }

}

impl InitTree {
    pub fn is_valid_tree() {

    }

    pub fn parse_tree() {
        
    }
}
