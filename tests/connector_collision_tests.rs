use archwiz::graph::build_graph;
use archwiz::layout;
use archwiz::scanner::scan_directory;
use std::collections::HashMap;
use std::fs;

fn tmp() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    p.push(format!(
        "archwiz_conn_{}_{}",
        std::process::id(),
        CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

fn assert_no_collisions(root: &std::path::Path, width: usize) {
    let scanned = scan_directory(root).unwrap();
    let g = build_graph(&scanned);
    let l = layout::compute(&g, width);

    // All nodes retained.
    assert_eq!(
        l.nodes.iter().filter(|n| n.is_some()).count(),
        g.nodes.len()
    );

    // Bounds.
    for r in l.nodes.iter().filter_map(|r| *r) {
        assert!(r.x + r.w <= width, "node overflow");
    }
    for s in &l.strokes {
        assert!(s.x < width, "stroke x overflow");
        assert!(s.y < l.height, "stroke y overflow");
    }

    // No stroke inside a node bounding box.
    for s in &l.strokes {
        for r in l.nodes.iter().filter_map(|r| *r) {
            if s.x >= r.x && s.x < r.x + r.w && s.y >= r.y && s.y < r.y + r.h {
                panic!(
                    "stroke {:?} at ({},{}) inside node rect {:?}",
                    s.ch, s.x, s.y, r
                );
            }
        }
    }

    // No cell with conflicting glyphs.
    let mut cells: HashMap<(usize, usize), char> = HashMap::new();
    for s in &l.strokes {
        if let Some(existing) = cells.get(&(s.x, s.y)) {
            // merged strokes should already be unique; any difference is a conflict
            if *existing != s.ch {
                panic!(
                    "conflicting glyphs at ({},{}): {} vs {}",
                    s.x, s.y, existing, s.ch
                );
            }
        } else {
            cells.insert((s.x, s.y), s.ch);
        }
    }
}

#[test]
fn wide_tree() {
    let root = tmp();
    for i in 1..=12 {
        fs::write(root.join(format!("f{}.rs", i)), "x").unwrap();
    }
    assert_no_collisions(&root, 90);
}

#[test]
fn deep_tree() {
    let root = tmp();
    let mut p = root.clone();
    for name in ["a", "b", "c", "d"] {
        p = p.join(name);
        fs::create_dir_all(&p).unwrap();
    }
    fs::write(p.join("x.rs"), "x").unwrap();
    assert_no_collisions(&root, 90);
}

#[test]
fn multiple_sibling_dirs() {
    let root = tmp();
    for d in 1..=4 {
        let dd = root.join(format!("dir{}", d));
        fs::create_dir_all(&dd).unwrap();
        for i in 1..=3 {
            fs::write(dd.join(format!("f{}.rs", i)), "x").unwrap();
        }
    }
    assert_no_collisions(&root, 80);
}

#[test]
fn narrow_terminal() {
    let root = tmp();
    for i in 1..=8 {
        fs::write(root.join(format!("longfile_{}.rs", i)), "x").unwrap();
    }
    assert_no_collisions(&root, 40);
}

#[test]
fn uneven_children() {
    let root = tmp();
    for d in 1..=3 {
        let dd = root.join(format!("d{}", d));
        fs::create_dir_all(&dd).unwrap();
        for i in 1..=5 {
            fs::write(dd.join(format!("f{}.rs", i)), "x").unwrap();
        }
    }
    for i in 1..=3 {
        fs::write(root.join(format!("r{}.rs", i)), "x").unwrap();
    }
    assert_no_collisions(&root, 90);
}

#[test]
fn branches_with_single_child() {
    let root = tmp();
    let mut p = root.clone();
    for name in ["a", "b", "c"] {
        p = p.join(name);
        fs::create_dir_all(&p).unwrap();
    }
    fs::write(p.join("x.rs"), "x").unwrap();
    assert_no_collisions(&root, 60);
}
