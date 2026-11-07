use archwiz::scanner::{ScanOptions, scan_directory, scan_directory_with};
use std::fs;
use std::path::Path;
// Use a simple temp dir helper without external crate.
fn tmp() -> std::path::PathBuf {
    static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let mut p = std::env::temp_dir();
    p.push(format!(
        "archwiz_test_{}_{}",
        std::process::id(),
        CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn empty_directory() {
    let root = tmp();
    let node = scan_directory(&root).unwrap();
    assert!(node.children.is_empty());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn nested_directories() {
    let root = tmp();
    fs::create_dir_all(root.join("a/b")).unwrap();
    fs::write(root.join("a/b/x.rs"), "fn main(){}").unwrap();
    let node = scan_directory(&root).unwrap();
    assert_eq!(node.children.len(), 1);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn files_with_spaces() {
    let root = tmp();
    fs::write(root.join("my file.txt"), "hi").unwrap();
    let node = scan_directory(&root).unwrap();
    assert_eq!(node.children.len(), 1);
    match &node.children[0] {
        archwiz::model::Node::File(f) => assert_eq!(f.name, "my file.txt"),
        _ => panic!("expected file"),
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn deterministic_ordering() {
    let root = tmp();
    for name in ["z.txt", "a.txt", "m.txt"] {
        fs::write(root.join(name), "").unwrap();
    }
    let n1 = scan_directory(&root).unwrap();
    let n2 = scan_directory(&root).unwrap();
    let names1: Vec<_> = n1
        .children
        .iter()
        .map(|c| match c {
            archwiz::model::Node::File(f) => f.name.clone(),
            archwiz::model::Node::Directory(d) => d.name.clone(),
            archwiz::model::Node::Group(g) => g.name.clone(),
        })
        .collect();
    let names2: Vec<_> = n2
        .children
        .iter()
        .map(|c| match c {
            archwiz::model::Node::File(f) => f.name.clone(),
            archwiz::model::Node::Directory(d) => d.name.clone(),
            archwiz::model::Node::Group(g) => g.name.clone(),
        })
        .collect();
    assert_eq!(names1, vec!["a.txt", "m.txt", "z.txt"]);
    assert_eq!(names1, names2);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn excluded_directories() {
    let root = tmp();
    fs::create_dir_all(root.join("target")).unwrap();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/lib.rs"), "").unwrap();
    let node = scan_directory(&root).unwrap();
    let names: Vec<_> = node
        .children
        .iter()
        .map(|c| match c {
            archwiz::model::Node::Directory(d) => d.name.clone(),
            archwiz::model::Node::Group(g) => g.name.clone(),
            archwiz::model::Node::File(f) => f.name.clone(),
        })
        .collect();
    assert!(!names.contains(&"target".to_string()));
    assert!(names.contains(&"src".to_string()));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn custom_exclude() {
    let root = tmp();
    fs::create_dir_all(root.join("foo")).unwrap();
    let mut opts = ScanOptions::default();
    opts.exclusions.insert("foo".to_string());
    let node = scan_directory_with(&root, &opts).unwrap();
    let names: Vec<_> = node
        .children
        .iter()
        .map(|c| match c {
            archwiz::model::Node::Directory(d) => d.name.clone(),
            archwiz::model::Node::Group(g) => g.name.clone(),
            archwiz::model::Node::File(f) => f.name.clone(),
        })
        .collect();
    assert!(!names.contains(&"foo".to_string()));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn symlinks_skipped() {
    let root = tmp();
    let target = tmp();
    fs::write(target.join("x.txt"), "").unwrap();
    #[cfg(unix)]
    let _ = std::os::unix::fs::symlink(&target, root.join("link"));
    let node = scan_directory(&root).unwrap();
    let names: Vec<_> = node
        .children
        .iter()
        .map(|c| match c {
            archwiz::model::Node::Directory(d) => d.name.clone(),
            archwiz::model::Node::Group(g) => g.name.clone(),
            archwiz::model::Node::File(f) => f.name.clone(),
        })
        .collect();
    assert!(!names.contains(&"link".to_string()));
    let _ = fs::remove_dir_all(&root);
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn depth_limit() {
    let root = tmp();
    fs::create_dir_all(root.join("a/b")).unwrap();
    fs::write(root.join("a/b/x.rs"), "").unwrap();
    let opts = archwiz::scanner::ScanOptions {
        max_depth: Some(1),
        ..ScanOptions::default()
    };
    let node = scan_directory_with(&root, &opts).unwrap();
    // root has 'a' dir with empty children
    let a = &node.children[0];
    match a {
        archwiz::model::Node::Directory(d) => assert!(d.children.is_empty()),
        _ => panic!("expected dir"),
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn missing_path() {
    let res = scan_directory(Path::new("/definitely/not/exist/xyz"));
    assert!(res.is_err());
}

#[test]
fn different_extensions() {
    let root = tmp();
    for name in ["main.rs", "app.py", "index.js", "Main.java"] {
        fs::write(root.join(name), "").unwrap();
    }
    let node = scan_directory(&root).unwrap();
    assert_eq!(node.children.len(), 4);
    let _ = fs::remove_dir_all(&root);
}
