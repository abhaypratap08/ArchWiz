use archwiz::graph::build_graph;
use archwiz::layout;
use archwiz::scanner::scan_directory;
use std::fs;

fn tmp() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    p.push(format!(
        "archwiz_bal_{}_{}",
        std::process::id(),
        CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn root_centered() {
    let root = tmp();
    fs::write(root.join("a.rs"), "").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let term = 80usize;
    let l = layout::compute(&g, term);
    let r = l.nodes[0].unwrap();
    let expected = (term - r.w) / 2;
    assert!((r.x as isize - expected as isize).abs() <= 1);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn parent_centered_over_child_group() {
    let root = tmp();
    fs::create_dir_all(root.join("src")).unwrap();
    for i in 0..4 {
        fs::write(root.join("src").join(format!("f{}.rs", i)), "").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 100);
    // find src node
    let src_id = g
        .nodes
        .iter()
        .find(|n| n.label == "src/")
        .map(|n| n.id)
        .unwrap();
    let src_rect = l.nodes[src_id].unwrap();
    let parent_center = src_rect.x + src_rect.w / 2;
    // children of src are grouped? 4 files < threshold? 4<6 so individual; their min/max centers
    let child_centers: Vec<usize> = g
        .edges
        .iter()
        .filter(|e| e.from == src_id)
        .map(|e| l.nodes[e.to].unwrap().x + l.nodes[e.to].unwrap().w / 2)
        .collect();
    if !child_centers.is_empty() {
        let min_c = *child_centers.iter().min().unwrap();
        let max_c = *child_centers.iter().max().unwrap();
        let group_center = (min_c + max_c) / 2;
        assert!((parent_center as isize - group_center as isize).abs() <= 4);
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn wide_graph_not_single_left_column() {
    let root = tmp();
    for i in 0..5 {
        fs::write(root.join(format!("f{}.rs", i)), "").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 200);
    let max_right = l
        .nodes
        .iter()
        .filter_map(|r| *r)
        .map(|r| r.x + r.w)
        .max()
        .unwrap();
    assert!(max_right > 50);
    let _ = fs::remove_dir_all(&root);
}
