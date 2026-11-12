use std::path::PathBuf;

pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MiB", bytes as f64 / (1024.0 * 1024.0))
    }
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
