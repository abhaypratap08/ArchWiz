use crate::classifier::{FileKind, classify_file};
use crate::model::{DirectoryNode, FileNode, GroupNode, Node};
use std::collections::BTreeMap;
use std::path::Path;

pub fn group_directory(mut dir: DirectoryNode) -> DirectoryNode {
    // Recurse first.
    let mut new_children = Vec::new();
    for child in dir.children.drain(..) {
        match child {
            Node::Directory(d) => new_children.push(Node::Directory(group_directory(d))),
            other => new_children.push(other),
        }
    }

    // Separate files from other nodes.
    let mut files: Vec<FileNode> = Vec::new();
    let mut others: Vec<Node> = Vec::new();
    for child in new_children {
        match child {
            Node::File(f) => files.push(f),
            other => others.push(other),
        }
    }

    // Group files by common prefix before '_' or '-'.
    let mut groups_map: BTreeMap<String, Vec<FileNode>> = BTreeMap::new();
    let mut ungrouped: Vec<FileNode> = Vec::new();
    for file in files {
        match file_prefix(&file.name) {
            Some(prefix) => groups_map.entry(prefix).or_default().push(file),
            None => ungrouped.push(file),
        }
    }

    let mut grouped_nodes: Vec<Node> = Vec::new();
    for (prefix, mut group_files) in groups_map {
        if group_files.len() >= 2 {
            group_files.sort_by(|a, b| a.name.cmp(&b.name));
            grouped_nodes.push(Node::Group(GroupNode {
                name: prefix,
                path: dir.path.clone(),
                files: group_files,
            }));
        } else {
            ungrouped.extend(group_files);
        }
    }

    // For large directories, group remaining files by classification.
    if ungrouped.len() > 6 {
        let mut by_kind: BTreeMap<FileKind, Vec<FileNode>> = BTreeMap::new();
        for file in ungrouped.drain(..) {
            by_kind.entry(classify_file(&file)).or_default().push(file);
        }
        for (kind, mut kind_files) in by_kind {
            if kind_files.len() >= 2 {
                kind_files.sort_by(|a, b| a.name.cmp(&b.name));
                let name = match kind {
                    FileKind::Source => "source",
                    FileKind::Test => "test",
                    FileKind::Configuration => "config",
                    FileKind::Documentation => "docs",
                    FileKind::Asset => "assets",
                    FileKind::Generated => "generated",
                    FileKind::Other => "other",
                };
                grouped_nodes.push(Node::Group(GroupNode {
                    name: name.to_string(),
                    path: dir.path.clone(),
                    files: kind_files,
                }));
            } else {
                ungrouped.extend(kind_files);
            }
        }
    }

    // Build final child list: others + groups + ungrouped files, sorted by display name.
    let mut result: Vec<Node> = others;
    result.extend(grouped_nodes);
    for f in ungrouped {
        result.push(Node::File(f));
    }

    // Sort by a stable display key.
    result.sort_by_key(node_sort_key);

    dir.children = result;
    dir
}

fn node_sort_key(node: &Node) -> String {
    match node {
        Node::Directory(d) => format!("0:{}", d.name),
        Node::Group(g) => format!("1:{}", g.name),
        Node::File(f) => format!("2:{}", f.name),
    }
}

fn file_prefix(name: &str) -> Option<String> {
    let stem = Path::new(name).file_stem()?.to_str()?;
    let pos = stem.find(['_', '-'])?;
    if pos == 0 {
        return None;
    }
    Some(stem[..pos].to_string())
}
