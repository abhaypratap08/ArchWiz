use archwiz::graph::build_graph;
use archwiz::layout;
use archwiz::scanner::scan_directory;
use std::fs;

fn tmp() -> std::path::PathBuf {
    static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let mut p = std::env::temp_dir();
    p.push(format!(
        "archwiz_layout_{}_{}",
        std::process::id(),
        CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

fn rects_overlap(a: &layout::NodeRect, b: &layout::NodeRect) -> bool {
    // overlap if intervals overlap on both axes
    let x_overlap = a.x < b.x + b.w && b.x < a.x + a.w;
    let y_overlap = a.y < b.y + b.h && b.y < a.y + a.h;
    x_overlap && y_overlap
}

#[test]
fn single_node() {
    let root = tmp();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 80);
    assert_eq!(l.nodes.len(), g.nodes.len());
    assert!(l.nodes[0].is_some());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn one_child() {
    let root = tmp();
    fs::write(root.join("a.rs"), "").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 80);
    assert_eq!(
        l.nodes.iter().filter(|n| n.is_some()).count(),
        g.nodes.len()
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn multiple_siblings_no_overlap() {
    let root = tmp();
    for i in 0..8 {
        fs::write(root.join(format!("f{}.rs", i)), "").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 100);
    let rects: Vec<_> = l.nodes.iter().filter_map(|r| *r).collect();
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(!rects_overlap(&rects[i], &rects[j]), "cards overlap");
        }
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn deep_nesting_retained() {
    let root = tmp();
    fs::create_dir_all(root.join("a/b/c/d")).unwrap();
    fs::write(root.join("a/b/c/d/x.rs"), "").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 120);
    assert_eq!(
        l.nodes.iter().filter(|n| n.is_some()).count(),
        g.nodes.len()
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn large_sibling_group_wraps() {
    let root = tmp();
    for i in 0..20 {
        fs::write(root.join(format!("file_{:02}.rs", i)), "").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 80);
    assert_eq!(
        l.nodes.iter().filter(|n| n.is_some()).count(),
        g.nodes.len()
    );
    // no horizontal overflow
    for r in l.nodes.iter().filter_map(|r| *r) {
        assert!(r.x + r.w <= 80, "overflow node");
    }
    for s in &l.strokes {
        assert!(s.x < 80, "stroke overflow");
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn narrow_terminal() {
    let root = tmp();
    for i in 0..5 {
        fs::write(root.join(format!("longfilename_{}.rs", i)), "").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 40);
    for r in l.nodes.iter().filter_map(|r| *r) {
        assert!(r.x + r.w <= 40, "overflow narrow");
    }
    for s in &l.strokes {
        assert!(s.x < 40, "stroke overflow narrow");
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn long_filenames() {
    let root = tmp();
    fs::write(
        root.join("a_very_long_filename_that_exceeds_normal_width.rs"),
        "",
    )
    .unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 60);
    for r in l.nodes.iter().filter_map(|r| *r) {
        assert!(r.x + r.w <= 60);
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn unicode_filenames() {
    let root = tmp();
    fs::write(root.join("файл.rs"), "").unwrap();
    fs::write(root.join("数据.txt"), "").unwrap();
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 80);
    assert_eq!(
        l.nodes.iter().filter(|n| n.is_some()).count(),
        g.nodes.len()
    );
    for r in l.nodes.iter().filter_map(|r| *r) {
        assert!(r.x + r.w <= 80);
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn no_card_overlaps_strokes_not_checked() {
    // connectors should not overlap unrelated cards; basic bound check
    let root = tmp();
    fs::create_dir_all(root.join("src")).unwrap();
    for i in 0..6 {
        fs::write(root.join(format!("f{}.rs", i)), "").unwrap();
    }
    for i in 0..6 {
        fs::write(root.join("src").join(format!("g{}.rs", i)), "").unwrap();
    }
    let scanned = scan_directory(&root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, 90);
    // strokes must lie within canvas
    for s in &l.strokes {
        assert!(s.x < 90);
        assert!(s.y < l.height);
    }
    let _ = fs::remove_dir_all(&root);
}
