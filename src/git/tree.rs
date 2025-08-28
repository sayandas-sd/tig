

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
    pub fn create_tree(content: Vec<InitTree>) -> Tree {
        let mut content_of_tree = String::new();
        for tree in &content {
            let tree_entry = Tree::get_content_of_tree(content_of_tree);
            content_of_tree.push_str(&tree_entry);

        }

        let size: i32 = String::from(content_of_tree.clone()).chars().count() as i32;

        Tree {
            content,
            content_size: size,
            data: content_of_tree,
        }
    }

    pub fn get_tree_content(content_of_tree: &String) -> Result<Tree, String>{

        let mut new_line: Vec<&str> = content_of_tree.split("\n").collect::<Vec<&str>>();
        new_line.pop();

        let content_size: i32 = content_of_tree.chars().count() as i32;

        let mut tree_contents: Vec<InitTree> = Vec::new();

        for line in new_line {
            let entry_content = line.split("\0").collect();

            println("{:#?}", entry_content);

            let valid_tree: bool = InitTree::is_valid_tree(&entry_content);

            if valid_tree {
                let tree_object: InitTree = InitTree::parse_tree(entry_content);

                tree_contents.push(tree_object);

            } else {
                return Err(format!("Invalid tree entry: {:?}", entry_parts));
            }
        }

        Ok(
            Tree {
                content: tree_contents,
                content_size,
                data: content_of_tree.to_string()
            }
        )

    }

    pub fn get_content_of_tree(init_tree: &InitTree) -> String {
        let mut content: String = String::new();

        match &init_tree.object {
            GitObject::Blob => {
                content.push_str("blob ");
            }
            GitObject::Tree => {
                content.push_str("tree ");
            }
        }

        content.push('\0');
        content.push_str(&init_tree.name);
        content.push_str(&init_tree.hash);
        content
    }

}

impl InitTree {
    pub fn is_valid_tree() {

    }

    pub fn parse_tree() {
        
    }
}
