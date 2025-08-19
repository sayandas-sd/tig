

#[derive(Debug, PartialEq, Clone)]
pub enum GitObject {
    Blob,
    Tree,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentTypes {
    UTF8,
    NonUTF8
}