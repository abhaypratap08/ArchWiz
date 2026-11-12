use crate::model::{DirectoryNode, Node};
use crate::statistics;
use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::collections::HashSet;
use std::io::{self, Write};

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
        let _ = crossterm::cursor::Show;
        let _ = write!(io::stdout(), "\x1b[0m");
        let _ = io::stdout().flush();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExplorerMode {
    Browse,
    Search,
    Stats,
}

#[derive(Clone)]
enum Item {
    Dir(DirectoryNode),
    Group(String, Vec<crate::model::FileNode>),
    File(crate::model::FileNode),
}

pub fn run(root: &DirectoryNode, use_color: bool) -> Result<(), String> {
    enable_raw_mode().map_err(|e| e.to_string())?;
    let _guard = TerminalGuard;
    io::stdout()
        .execute(EnterAlternateScreen)
        .map_err(|e| e.to_string())?;

    let mut current = root.clone();
    let mut stack: Vec<DirectoryNode> = Vec::new();
    let mut selected: usize = 0;
    let mut expanded: HashSet<String> = HashSet::new();
    let mut search: String = String::new();
    let mut mode = ExplorerMode::Browse;
    let mut message: String = String::new();

    loop {
        let items = build_items(&current, &expanded, &search);
        if selected >= items.len() && !items.is_empty() {
            selected = items.len() - 1;
        }

        let searching = mode == ExplorerMode::Search;
        let show_stats = mode == ExplorerMode::Stats;

        draw_screen(
            &current, &items, selected, searching, &search, show_stats, &message, use_color,
        );
        message.clear();

        match event::read() {
            Ok(Event::Key(KeyEvent {
                code, modifiers, ..
            })) => {
                if searching {
                    match code {
                        KeyCode::Enter => {
                            mode = ExplorerMode::Browse;
                        }
                        KeyCode::Esc => {
                            mode = ExplorerMode::Browse;
                            search.clear();
                        }
                        KeyCode::Backspace => {
                            search.pop();
                        }
                        KeyCode::Char(c) => {
                            search.push(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Esc => {
                        if show_stats {
                            mode = ExplorerMode::Browse;
                        } else {
                            break;
                        }
                    }
                    KeyCode::Char('s') => {
                        mode = if show_stats {
                            ExplorerMode::Browse
                        } else {
                            ExplorerMode::Stats
                        };
                    }
                    KeyCode::Char('/') => {
                        mode = ExplorerMode::Search;
                        search.clear();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected + 1 < items.len() {
                            selected += 1;
                        }
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        if selected + 1 < items.len() {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(item) = items.get(selected) {
                            match item {
                                Item::Dir(d) => {
                                    stack.push(current.clone());
                                    current = d.clone();
                                    selected = 0;
                                }
                                Item::Group(name, _) => {
                                    let key = format!("{}/{}", current.name, name);
                                    if expanded.contains(&key) {
                                        expanded.remove(&key);
                                    } else {
                                        expanded.insert(key);
                                    }
                                }
                                Item::File(f) => {
                                    message = format!("File: {} ({} bytes)", f.name, f.size_bytes);
                                }
                            }
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(Item::Group(name, _)) = items.get(selected) {
                            let key = format!("{}/{}", current.name, name);
                            if expanded.contains(&key) {
                                expanded.remove(&key);
                            } else {
                                expanded.insert(key);
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(parent) = stack.pop() {
                            current = parent;
                            selected = 0;
                        }
                    }
                    KeyCode::Char('i') => {
                        if let Some(item) = items.get(selected) {
                            message = match item {
                                Item::Dir(d) => {
                                    format!("Directory: {} ({} children)", d.name, d.children.len())
                                }
                                Item::Group(name, files) => {
                                    format!("Group: {} ({} files)", name, files.len())
                                }
                                Item::File(f) => {
                                    format!("File: {} ({} bytes)", f.name, f.size_bytes)
                                }
                            };
                        }
                    }
                    _ => {}
                }
                if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                    break;
                }
            }
            Ok(Event::Resize(_, _)) => {}
            _ => {}
        }
    }

    Ok(())
}

fn build_items(dir: &DirectoryNode, expanded: &HashSet<String>, search: &str) -> Vec<Item> {
    let mut items = Vec::new();
    for child in &dir.children {
        match child {
            Node::Directory(d) => {
                let item = Item::Dir(d.clone());
                if matches_search(&item, search) {
                    items.push(item);
                }
            }
            Node::File(f) => {
                let item = Item::File(f.clone());
                if matches_search(&item, search) {
                    items.push(item);
                }
            }
            Node::Group(g) => {
                let item = Item::Group(g.name.clone(), g.files.clone());
                if matches_search(&item, search) {
                    items.push(item.clone());
                }
                let key = format!("{}/{}", dir.name, g.name);
                if expanded.contains(&key) {
                    for f in &g.files {
                        let fi = Item::File(f.clone());
                        if matches_search(&fi, search) {
                            items.push(fi);
                        }
                    }
                }
            }
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DirectoryNode, FileNode, Node};
    use std::path::PathBuf;

    #[test]
    fn build_items_filters_search() {
        let dir = DirectoryNode {
            name: "root".into(),
            path: PathBuf::from("/tmp"),
            children: vec![
                Node::File(FileNode {
                    name: "main.rs".into(),
                    path: PathBuf::from("/tmp/main.rs"),
                    size_bytes: 1,
                }),
                Node::File(FileNode {
                    name: "lib.rs".into(),
                    path: PathBuf::from("/tmp/lib.rs"),
                    size_bytes: 1,
                }),
            ],
        };
        let expanded = HashSet::new();
        let items = build_items(&dir, &expanded, "main");
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn build_items_empty_search_returns_all() {
        let dir = DirectoryNode {
            name: "root".into(),
            path: PathBuf::from("/tmp"),
            children: vec![
                Node::File(FileNode {
                    name: "a.rs".into(),
                    path: PathBuf::from("/tmp/a.rs"),
                    size_bytes: 1,
                }),
                Node::File(FileNode {
                    name: "b.rs".into(),
                    path: PathBuf::from("/tmp/b.rs"),
                    size_bytes: 1,
                }),
            ],
        };
        let expanded = HashSet::new();
        let items = build_items(&dir, &expanded, "");
        assert_eq!(items.len(), 2);
    }
}
fn matches_search(item: &Item, search: &str) -> bool {
    if search.is_empty() {
        return true;
    }
    let lower = search.to_lowercase();
    match item {
        Item::Dir(d) => d.name.to_lowercase().contains(&lower),
        Item::Group(name, _) => name.to_lowercase().contains(&lower),
        Item::File(f) => f.name.to_lowercase().contains(&lower),
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_screen(
    current: &DirectoryNode,
    items: &[Item],
    selected: usize,
    searching: bool,
    search: &str,
    show_stats: bool,
    message: &str,
    use_color: bool,
) {
    let _ = use_color;
    let mut out = String::new();
    out.push_str("\x1b[2J\x1b[H");
    if use_color {
        out.push_str("  \x1b[1;38;5;51m◈ ARCHWIZ\x1b[0m  \x1b[38;5;109m/\x1b[0m  \x1b[1mINTERACTIVE\x1b[0m\n");
    } else {
        out.push_str("  ◈ ARCHWIZ  /  INTERACTIVE\n");
    }
    out.push_str(&format!("  Path: {}\n", current.path.display()));
    out.push_str("  ─────────────────────────────────────────\n");

    if show_stats {
        let stats = statistics::collect(current);
        // Simple inline stats.
        out.push_str(&format!(
            "  Files: {}  Dirs: {}  Size: {}\n",
            stats.total_files, stats.total_dirs, stats.total_size
        ));
        out.push_str("  (press 's' to toggle, 'q' to quit)\n");
    } else {
        for (i, item) in items.iter().enumerate() {
            let marker = if i == selected { ">" } else { " " };
            let line = match item {
                Item::Dir(d) => format!("{} [DIR]  {}", marker, d.name),
                Item::Group(name, files) => {
                    format!("{} [GRP]  {} ({} files)", marker, name, files.len())
                }
                Item::File(f) => format!("{} [FILE] {}", marker, f.name),
            };
            if use_color && i == selected {
                out.push_str(&format!("  \x1b[1;38;5;51m{}\x1b[0m\n", line));
            } else {
                out.push_str(&format!("  {}\n", line));
            }
        }
        if items.is_empty() {
            out.push_str("  (empty)\n");
        }
    }

    if searching {
        out.push_str(&format!("\n  Search: {}\n", search));
    }
    if !message.is_empty() {
        out.push_str(&format!("\n  {}\n", message));
    }
    out.push_str("\n  h/j/k/l move  Enter open  Space expand  / search  i inspect  s stats  Backspace parent  q quit\n");

    let _ = write!(io::stdout(), "{}", out);
    let _ = io::stdout().flush();
}
