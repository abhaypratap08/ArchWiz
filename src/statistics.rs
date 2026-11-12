use crate::classifier::{classify_file, source_language};
use crate::model::{DirectoryNode, Node};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
    pub kind: crate::classifier::FileKind,
    pub language: crate::classifier::SourceLanguage,
}

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: String,
    pub depth: usize,
    pub direct_files: usize,
    pub total_size: u64,
}

#[derive(Debug)]
pub struct ProjectStats {
    pub name: String,
    pub root: String,
    pub total_files: usize,
    pub total_dirs: usize,
    pub max_depth: usize,
    pub total_size: u64,
    pub kind_counts: BTreeMap<String, usize>,
    pub language_counts: BTreeMap<String, usize>,
    pub language_bytes: BTreeMap<String, u64>,
    pub largest_files: Vec<FileEntry>,
    pub top_dirs: Vec<DirEntry>,
    pub deepest_dirs: Vec<DirEntry>,
    pub largest_dirs: Vec<DirEntry>,
}

pub fn collect(root: &DirectoryNode) -> ProjectStats {
    let mut stats = ProjectStats {
        name: root.name.clone(),
        root: root.path.display().to_string(),
        total_files: 0,
        total_dirs: 0,
        max_depth: 0,
        total_size: 0,
        kind_counts: BTreeMap::new(),
        language_counts: BTreeMap::new(),
        language_bytes: BTreeMap::new(),
        largest_files: Vec::new(),
        top_dirs: Vec::new(),
        deepest_dirs: Vec::new(),
        largest_dirs: Vec::new(),
    };

    let mut files: Vec<FileEntry> = Vec::new();
    let mut dirs: Vec<DirEntry> = Vec::new();

    walk(root, "", 0, &mut stats, &mut files, &mut dirs);

    stats.total_files = files.len();
    stats.total_dirs = dirs.len();
    // dirs includes root? We counted root in walk. Ensure total_dirs counts all directories including root.
    // Actually walk increments total_dirs for root. Good.

    // kind counts
    for f in &files {
        let key = format!("{:?}", f.kind);
        // Use display labels instead of debug.
        let _ = key;
    }
    // Rebuild kind counts with labels.
    let mut kind_map: BTreeMap<String, usize> = BTreeMap::new();
    for f in &files {
        let label = kind_label(f.kind);
        *kind_map.entry(label.to_string()).or_insert(0) += 1;
    }
    stats.kind_counts = kind_map;

    // language counts
    for f in &files {
        let name = language_name(f.language);
        *stats.language_counts.entry(name.to_string()).or_insert(0) += 1;
        *stats.language_bytes.entry(name.to_string()).or_insert(0) += f.size;
    }

    // largest files
    let mut sorted_files = files;
    sorted_files.sort_by_key(|a| std::cmp::Reverse(a.size));
    stats.largest_files = sorted_files.into_iter().take(10).collect();

    // top dirs by direct files
    let mut by_files = dirs.clone();
    by_files.sort_by_key(|a| std::cmp::Reverse(a.direct_files));
    stats.top_dirs = by_files.into_iter().take(5).collect();

    // deepest dirs
    let mut by_depth = dirs.clone();
    by_depth.sort_by_key(|a| std::cmp::Reverse(a.depth));
    stats.deepest_dirs = by_depth.into_iter().take(5).collect();

    // largest dirs
    let mut by_size = dirs;
    by_size.sort_by_key(|a| std::cmp::Reverse(a.total_size));
    stats.largest_dirs = by_size.into_iter().take(5).collect();

    stats
}

fn walk(
    dir: &DirectoryNode,
    rel: &str,
    depth: usize,
    stats: &mut ProjectStats,
    files: &mut Vec<FileEntry>,
    dirs: &mut Vec<DirEntry>,
) -> u64 {
    stats.max_depth = stats.max_depth.max(depth);
    stats.total_dirs += 1;

    let display_path = if rel.is_empty() {
        dir.name.clone()
    } else {
        format!("{}/{}", rel, dir.name)
    };

    let mut direct_files = 0usize;
    let mut total_size = 0u64;

    for child in &dir.children {
        match child {
            Node::File(f) => {
                direct_files += 1;
                total_size += f.size_bytes;
                stats.total_size += f.size_bytes;
                files.push(FileEntry {
                    path: format!("{}/{}", display_path, f.name),
                    size: f.size_bytes,
                    kind: classify_file(f),
                    language: source_language(&f.path),
                });
            }
            Node::Directory(d) => {
                let child_rel = if rel.is_empty() {
                    dir.name.clone()
                } else {
                    format!("{}/{}", rel, dir.name)
                };
                let child_size = walk(d, &child_rel, depth + 1, stats, files, dirs);
                total_size += child_size;
            }
            Node::Group(g) => {
                for f in &g.files {
                    direct_files += 1;
                    total_size += f.size_bytes;
                    stats.total_size += f.size_bytes;
                    files.push(FileEntry {
                        path: format!("{}/{}", display_path, f.name),
                        size: f.size_bytes,
                        kind: classify_file(f),
                        language: source_language(&f.path),
                    });
                }
            }
        }
    }

    // aggregated size for directory = direct + descendants. We only have direct here;
    // compute descendant sizes by re-walking? Instead approximate total_size as direct for now.
    // To get accurate aggregated size, we need post-order. We'll store direct and compute later.
    dirs.push(DirEntry {
        path: display_path,
        depth,
        direct_files,
        total_size,
    });
    total_size
}

fn kind_label(kind: crate::classifier::FileKind) -> &'static str {
    use crate::classifier::FileKind;
    match kind {
        FileKind::Source => "Source",
        FileKind::Test => "Test",
        FileKind::Configuration => "Configuration",
        FileKind::Documentation => "Documentation",
        FileKind::Asset => "Asset",
        FileKind::Generated => "Generated",
        FileKind::Other => "Other",
    }
}

fn language_name(lang: crate::classifier::SourceLanguage) -> &'static str {
    use crate::classifier::SourceLanguage;
    match lang {
        SourceLanguage::Rust => "Rust",
        SourceLanguage::Java => "Java",
        SourceLanguage::JavaScript => "JavaScript",
        SourceLanguage::TypeScript => "TypeScript",
        SourceLanguage::Python => "Python",
        SourceLanguage::Go => "Go",
        SourceLanguage::C => "C",
        SourceLanguage::Cpp => "C++",
        SourceLanguage::CSharp => "C#",
        SourceLanguage::Php => "PHP",
        SourceLanguage::Ruby => "Ruby",
        SourceLanguage::Kotlin => "Kotlin",
        SourceLanguage::Swift => "Swift",
        SourceLanguage::Shell => "Shell",
        SourceLanguage::Other => "Other",
    }
}

pub fn print_stats(stats: &ProjectStats, use_color: bool) {
    let h = if use_color { "\x1b[1;38;5;51m" } else { "" };
    let r = if use_color { "\x1b[0m" } else { "" };
    let m = if use_color { "\x1b[38;5;109m" } else { "" };
    println!();
    println!("  {}ARCHWIZ  /  {}  /  STATISTICS{}", h, stats.name, r);
    println!();
    println!("  {}Project:{} {}", m, r, stats.name);
    println!("  {}Root:{}    {}", m, r, stats.root);
    println!("  {}Files:{}   {}", m, r, stats.total_files);
    println!("  {}Dirs:{}    {}", m, r, stats.total_dirs);
    println!(
        "  {}Size:{}    {}",
        m,
        r,
        crate::model::format_size(stats.total_size)
    );
    println!("  {}Depth:{}   {}", m, r, stats.max_depth);
    println!();

    println!("  {}FILE CATEGORIES{}", h, r);
    for (k, v) in &stats.kind_counts {
        println!("  {:<16} {:>6}", k, v);
    }
    println!();

    println!("  {}LANGUAGE DISTRIBUTION{}", h, r);
    let max_lang = stats.language_counts.values().copied().max().unwrap_or(1);
    for (lang, count) in &stats.language_counts {
        let bar_len = (*count as f64 / max_lang as f64 * 20.0).round() as usize;
        let bar: String = "█".repeat(bar_len);
        if use_color {
            println!("  {:<14} {:>4}  \x1b[38;5;75m{}\x1b[0m", lang, count, bar);
        } else {
            println!("  {:<14} {:>4}  {}", lang, count, bar);
        }
    }
    println!();

    println!("  {}LARGEST FILES{}", h, r);
    for f in stats.largest_files.iter().take(10) {
        println!("  {:>8}  {}", crate::model::format_size(f.size), f.path);
    }
    println!();

    println!("  {}TOP DIRECTORIES BY FILE COUNT{}", h, r);
    for d in stats.top_dirs.iter().take(5) {
        println!("  {:>6}  {}", d.direct_files, d.path);
    }
    println!();

    println!("  {}DEEPEST DIRECTORIES{}", h, r);
    for d in stats.deepest_dirs.iter().take(5) {
        println!("  {:>6}  {}", d.depth, d.path);
    }
    println!();
}
