use archwiz::graph::build_graph;
use archwiz::model::DirectoryNode;
use archwiz::scanner::scan_directory;
use std::fs;

fn tmp() -> std::path::PathBuf {
    static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let mut p = std::env::temp_dir();
    p.push(format!(
        "archwiz_graph_{}_{}",
        std::process::id(),
        CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn empty_minimal() {
    let root = tmp();
    let dir = DirectoryNode {
        name: "proj".into(),
        path: root.clone(),
        children: vec![],
    };
    let g = build_graph(&dir);
    assert_eq!(g.nodes.len(), 1);
    assert_eq!(g.edges.len(), 0);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn root_and_edges() {
    let root = tmp();
    fs::write(root.join("a.rs"), "x").unwrap();
    fs::create_dir_all(root.join("src")).unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    // root + file + dir
    assert_eq!(g.nodes.len(), 3);
    assert_eq!(g.edges.len(), 2);
    // root is project
    assert!(matches!(
        g.nodes[0].kind,
        archwiz::graph::GraphNodeKind::Project
    ));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn no_invalid_indexes() {
    let root = tmp();
    fs::write(root.join("a.rs"), "x").unwrap();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/b.rs"), "x").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    for e in &g.edges {
        assert!(e.from < g.nodes.len());
        assert!(e.to < g.nodes.len());
        assert_ne!(e.from, e.to);
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn parent_child_relationships() {
    let root = tmp();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/main.rs"), "x").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    // find src node
    let src_id = g
        .nodes
        .iter()
        .find(|n| n.label == "src/")
        .map(|n| n.id)
        .unwrap();
    // edge from root(0) to src
    assert!(g.edges.iter().any(|e| e.from == 0 && e.to == src_id));
    // file parent is src
    let file_id = g
        .nodes
        .iter()
        .find(|n| n.label == "main.rs")
        .map(|n| n.id)
        .unwrap();
    assert!(g.edges.iter().any(|e| e.from == src_id && e.to == file_id));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn classifier_file_kind_in_graph() {
    let root = tmp();
    fs::write(root.join("README.md"), "x").unwrap();
    fs::write(root.join("Cargo.toml"), "x").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let readme = g.nodes.iter().find(|n| n.label == "README.md").unwrap();
    assert!(matches!(
        readme.kind,
        archwiz::graph::GraphNodeKind::File(archwiz::classifier::FileKind::Documentation)
    ));
    let cargo = g.nodes.iter().find(|n| n.label == "Cargo.toml").unwrap();
    assert!(matches!(
        cargo.kind,
        archwiz::graph::GraphNodeKind::File(archwiz::classifier::FileKind::Configuration)
    ));
    let _ = fs::remove_dir_all(&root);
}
