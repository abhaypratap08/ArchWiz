use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub root: DirectoryNode,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectoryNode {
    pub name: String,
    pub path: PathBuf,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Directory(DirectoryNode),
    File(FileNode),
    Group(GroupNode),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GroupNode {
    pub name: String,
    pub path: PathBuf,
    pub files: Vec<FileNode>,
}
