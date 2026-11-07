use archwiz::graph::build_graph;
use archwiz::grouper::group_directory;
use archwiz::layout;
use archwiz::scanner::scan_directory;
use std::fs;

fn tmp() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    p.push(format!(
        "archwiz_group_{}_{}",
        std::process::id(),
        CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn common_prefix_grouping() {
    let root = tmp();
    for n in [
        "probe_centre.mjs",
        "probe_cqb.mjs",
        "probe_grade.mjs",
        "probe_scope.mjs",
    ] {
        fs::write(root.join(n), "x").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let grouped = group_directory(scanned);
    assert_eq!(grouped.children.len(), 1);
    match &grouped.children[0] {
        archwiz::model::Node::Group(g) => {
            assert_eq!(g.name, "probe");
            assert_eq!(g.files.len(), 4);
        }
        other => panic!("expected group, got {:?}", other),
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn unrelated_files_remain_separate() {
    let root = tmp();
    fs::write(root.join("vite.config.mjs"), "x").unwrap();
    fs::write(root.join("README.md"), "x").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let grouped = group_directory(scanned);
    assert_eq!(grouped.children.len(), 2);
    assert!(
        grouped
            .children
            .iter()
            .all(|c| matches!(c, archwiz::model::Node::File(_)))
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn different_extensions_handled() {
    let root = tmp();
    fs::write(root.join("data_one.json"), "x").unwrap();
    fs::write(root.join("data_two.toml"), "x").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let grouped = group_directory(scanned);
    // Different extensions but same prefix "data" -> grouped
    assert_eq!(grouped.children.len(), 1);
    match &grouped.children[0] {
        archwiz::model::Node::Group(g) => assert_eq!(g.files.len(), 2),
        _ => panic!("expected group"),
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn large_group_wraps() {
    let root = tmp();
    for i in 0..20 {
        fs::write(root.join(format!("item_{}.rs", i)), "x").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 60);
    // group node retained
    assert!(
        g.nodes
            .iter()
            .any(|n| matches!(n.kind, archwiz::graph::GraphNodeKind::Group))
    );
    for r in l.nodes.iter().filter_map(|r| *r) {
        assert!(r.x + r.w <= 60);
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn stable_ordering() {
    let root = tmp();
    for n in ["probe_b.mjs", "probe_a.mjs", "probe_c.mjs"] {
        fs::write(root.join(n), "x").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let grouped = group_directory(scanned);
    match &grouped.children[0] {
        archwiz::model::Node::Group(g) => {
            let names: Vec<_> = g.files.iter().map(|f| f.name.clone()).collect();
            assert_eq!(names, vec!["probe_a.mjs", "probe_b.mjs", "probe_c.mjs"]);
        }
        _ => panic!("expected group"),
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn aggregate_count_and_metadata() {
    let root = tmp();
    fs::write(root.join("probe_x.mjs"), "hello").unwrap();
    fs::write(root.join("probe_y.mjs"), "world!!").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let group_node = g
        .nodes
        .iter()
        .find(|n| matches!(n.kind, archwiz::graph::GraphNodeKind::Group))
        .unwrap();
    assert!(group_node.subtitle.contains("2 files"));
    assert_eq!(group_node.details.len(), 2);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn no_connector_overlap_around_group() {
    let root = tmp();
    for i in 0..5 {
        fs::write(root.join(format!("grp_{}.rs", i)), "x").unwrap();
    }
    fs::write(root.join("other.rs"), "x").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 80);
    // no stroke inside node rect
    for s in &l.strokes {
        for r in l.nodes.iter().filter_map(|r| *r) {
            assert!(
                !(s.x >= r.x && s.x < r.x + r.w && s.y >= r.y && s.y < r.y + r.h),
                "stroke inside rect"
            );
        }
    }
    let _ = fs::remove_dir_all(&root);
}
