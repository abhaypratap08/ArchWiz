use crate::classifier::{FileKind, classify_file};
use crate::grouper::group_directory;
use crate::model::{DirectoryNode, FileNode, Node};
use crate::theme;

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct GraphNode {
    pub id: usize,
    pub label: String,
    pub subtitle: String,
    pub kind: GraphNodeKind,
    pub details: Vec<String>,
    pub detail_colors: Vec<u8>,
    pub aggregate_size: u64,
    pub direct_children: usize,
    pub subdirectories: usize,
}

#[derive(Debug)]
pub struct GraphEdge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
pub enum GraphNodeKind {
    Project,
    Directory,
    File(FileKind),
    Group,
}

pub fn build_graph(root: &DirectoryNode) -> Graph {
    let grouped_root = group_directory(root.clone());
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    graph.nodes.push(GraphNode {
        id: 0,
        label: grouped_root.name.clone(),
        subtitle: "PROJECT ROOT".to_string(),
        kind: GraphNodeKind::Project,
        details: Vec::new(),
        detail_colors: Vec::new(),
        aggregate_size: 0,
        direct_children: grouped_root.children.len(),
        subdirectories: count_subdirs(&grouped_root),
    });

    add_children(&grouped_root, 0, &mut graph);
    graph
}

fn add_children(directory: &DirectoryNode, parent_id: usize, graph: &mut Graph) -> u64 {
    let mut aggregate_size = 0u64;

    for child in &directory.children {
        let id = graph.nodes.len();

        match child {
            Node::Directory(dir) => {
                graph.nodes.push(GraphNode {
                    id,
                    label: format!("{}/", dir.name),
                    subtitle: String::new(),
                    kind: GraphNodeKind::Directory,
                    details: Vec::new(),
                    detail_colors: Vec::new(),
                    aggregate_size: 0,
                    direct_children: dir.children.len(),
                    subdirectories: count_subdirs(dir),
                });

                graph.edges.push(GraphEdge {
                    from: parent_id,
                    to: id,
                });

                let child_size = add_children(dir, id, graph);

                let dir_node = graph.nodes.get_mut(id).unwrap();
                dir_node.subtitle = format!(
                    "DIRECTORY · {} files · {} dirs",
                    count_direct_files(dir),
                    count_subdirs(dir)
                );
                dir_node.aggregate_size = child_size;

                aggregate_size += child_size;
            }

            Node::File(file) => {
                graph.nodes.push(GraphNode {
                    id,
                    label: file.name.clone(),
                    subtitle: format!(
                        "{} · {}",
                        file_kind_label(file),
                        format_size(file.size_bytes)
                    ),
                    kind: GraphNodeKind::File(classify_file(file)),
                    details: Vec::new(),
                    detail_colors: Vec::new(),
                    aggregate_size: file.size_bytes,
                    direct_children: 0,
                    subdirectories: 0,
                });

                graph.edges.push(GraphEdge {
                    from: parent_id,
                    to: id,
                });

                aggregate_size += file.size_bytes;
            }

            Node::Group(group) => {
                let mut details = Vec::new();
                let mut detail_colors = Vec::new();
                let mut group_size = 0u64;
                for file in &group.files {
                    let kind = classify_file(file);
                    details.push(format!(
                        "{}  [{}]  {}",
                        file.name,
                        file_kind_label_for(kind),
                        format_size(file.size_bytes)
                    ));
                    detail_colors.push(theme::kind_color(&GraphNodeKind::File(kind)));
                    group_size += file.size_bytes;
                }

                graph.nodes.push(GraphNode {
                    id,
                    label: format!("{}/", group.name),
                    subtitle: format!(
                        "GROUP · {} files · {}",
                        group.files.len(),
                        format_size(group_size)
                    ),
                    kind: GraphNodeKind::Group,
                    details,
                    detail_colors,
                    aggregate_size: group_size,
                    direct_children: group.files.len(),
                    subdirectories: 0,
                });

                graph.edges.push(GraphEdge {
                    from: parent_id,
                    to: id,
                });

                aggregate_size += group_size;
            }
        }
    }

    aggregate_size
}

fn count_subdirs(dir: &DirectoryNode) -> usize {
    let mut count = 0;
    for child in &dir.children {
        if matches!(child, Node::Directory(_)) {
            count += 1;
        }
    }
    count
}

fn count_direct_files(dir: &DirectoryNode) -> usize {
    dir.children
        .iter()
        .filter(|c| matches!(c, Node::File(_)))
        .count()
}

fn file_kind_label(file: &FileNode) -> &'static str {
    match classify_file(file) {
        FileKind::Source => "SOURCE",
        FileKind::Test => "TEST",
        FileKind::Configuration => "CONFIG",
        FileKind::Documentation => "DOCS",
        FileKind::Asset => "ASSET",
        FileKind::Generated => "GEN",
        FileKind::Other => "OTHER",
    }
}

fn file_kind_label_for(kind: FileKind) -> &'static str {
    match kind {
        FileKind::Source => "SOURCE",
        FileKind::Test => "TEST",
        FileKind::Configuration => "CONFIG",
        FileKind::Documentation => "DOCS",
        FileKind::Asset => "ASSET",
        FileKind::Generated => "GEN",
        FileKind::Other => "OTHER",
    }
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MiB", bytes as f64 / (1024.0 * 1024.0))
    }
}

#[allow(dead_code)]
pub fn kind_label(kind: &GraphNodeKind) -> &'static str {
    match kind {
        GraphNodeKind::Project => "PROJECT",
        GraphNodeKind::Directory => "DIRECTORY",
        GraphNodeKind::File(fk) => match fk {
            FileKind::Source => "SOURCE",
            FileKind::Test => "TEST",
            FileKind::Configuration => "CONFIG",
            FileKind::Documentation => "DOCS",
            FileKind::Asset => "ASSET",
            FileKind::Generated => "GEN",
            FileKind::Other => "OTHER",
        },
        GraphNodeKind::Group => "GROUP",
    }
}
