use crate::model::{DirectoryNode, FileNode, Node};
use ignore::gitignore::Gitignore;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub max_depth: Option<usize>,
    pub exclusions: HashSet<String>,
    pub exclude_extensions: HashSet<String>,
    pub respect_gitignore: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        let mut exclusions = HashSet::new();
        for name in [
            ".git",
            "target",
            "node_modules",
            "dist",
            "build",
            "__pycache__",
            ".pytest_cache",
            ".mypy_cache",
            ".ruff_cache",
            ".venv",
            "venv",
            ".next",
            ".nuxt",
            ".turbo",
            "coverage",
        ] {
            exclusions.insert(name.to_string());
        }

        let mut exclude_extensions = HashSet::new();
        for ext in ["pyc", "pyo", "class", "o", "obj", "so", "dll", "exe", "log"] {
            exclude_extensions.insert(ext.to_string());
        }

        Self {
            max_depth: None,
            exclusions,
            exclude_extensions,
            respect_gitignore: true,
        }
    }
}

#[allow(dead_code)]
pub fn scan_directory(path: &Path) -> io::Result<DirectoryNode> {
    scan_directory_with(path, &ScanOptions::default())
}

pub fn scan_directory_with(path: &Path, options: &ScanOptions) -> io::Result<DirectoryNode> {
    scan_recursive(path, options, 0, &[])
}

fn scan_recursive(
    path: &Path,
    options: &ScanOptions,
    depth: usize,
    parent_ignores: &[Gitignore],
) -> io::Result<DirectoryNode> {
    if let Some(max) = options.max_depth
        && depth >= max
    {
        return Ok(DirectoryNode {
            name: path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string()),
            path: path.to_path_buf(),
            children: Vec::new(),
        });
    }

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());

    // Load .gitignore in this directory if present.
    let mut ignores: Vec<Gitignore> = parent_ignores.to_vec();
    if options.respect_gitignore {
        let gi_path = path.join(".gitignore");
        if gi_path.is_file() {
            let (g, _) = Gitignore::new(&gi_path);
            ignores.push(g);
        }
    }

    let mut children = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(path)?.collect::<io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }

        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let entry_name_str = entry_name.to_string_lossy();

        // Gitignore check.
        if options.respect_gitignore {
            let mut ignored = false;
            for gi in &ignores {
                let m = gi.matched(&entry_path, file_type.is_dir());
                if m.is_ignore() {
                    ignored = true;
                    break;
                }
            }
            if ignored {
                continue;
            }
        }

        if file_type.is_dir() {
            if options.exclusions.contains(entry_name_str.as_ref()) {
                continue;
            }
            let directory = scan_recursive(&entry_path, options, depth + 1, &ignores)?;
            children.push(Node::Directory(directory));
        } else if file_type.is_file() {
            // Extension filter.
            let ext = entry_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if options.exclude_extensions.contains(&ext) {
                continue;
            }

            let size_bytes = fs::metadata(&entry_path).map(|m| m.len()).unwrap_or(0);
            children.push(Node::File(FileNode {
                name: entry_name.to_string_lossy().into_owned(),
                path: entry_path,
                size_bytes,
            }));
        }
    }

    Ok(DirectoryNode {
        name,
        path: path.to_path_buf(),
        children,
    })
}
