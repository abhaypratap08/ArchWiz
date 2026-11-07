use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "archwiz",
    version,
    about = "Terminal-native software architecture explorer"
)]
pub struct Cli {
    /// Path to scan (defaults to current directory)
    pub path: Option<PathBuf>,

    /// Limit directory depth
    #[arg(short, long)]
    pub depth: Option<usize>,

    /// Exclude directories (comma-separated)
    #[arg(short, long)]
    pub exclude: Option<String>,

    /// Disable terminal colors
    #[arg(long)]
    pub no_color: bool,

    /// Override terminal width
    #[arg(long)]
    pub width: Option<usize>,

    /// Print project statistics
    #[arg(long)]
    pub stats: bool,

    /// Interactive exploration mode
    #[arg(long)]
    pub interactive: bool,

    /// Show every file entry without summarizing large groups
    #[arg(long)]
    pub full: bool,

    /// Maximum filenames shown inside a group preview (default 3)
    #[arg(long)]
    pub preview: Option<usize>,
}

impl Cli {
    pub fn exclusions(&self) -> std::collections::HashSet<String> {
        let mut set = std::collections::HashSet::new();
        if let Some(raw) = &self.exclude {
            for part in raw.split(',') {
                let trimmed = part.trim();
                if !trimmed.is_empty() {
                    set.insert(trimmed.to_string());
                }
            }
        }
        set
    }
}
