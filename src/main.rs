mod classifier;
mod cli;
mod graph;
mod grouper;
mod interactive;
mod layout;
mod model;
mod renderer;
mod scanner;
mod statistics;
mod theme;

use clap::Parser;
use cli::Cli;
use scanner::ScanOptions;
use std::process;

fn main() {
    let args = Cli::parse();

    if let Err(err) = run(args) {
        eprintln!("archwiz: {err}");
        process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), String> {
    let target = args.path.clone().unwrap_or_else(|| ".".into());

    let root_path = std::fs::canonicalize(&target)
        .map_err(|e| format!("cannot access '{}': {e}", target.display()))?;

    if !root_path.is_dir() {
        return Err(format!("'{}' is not a directory", root_path.display()));
    }

    let mut options = ScanOptions::default();
    if let Some(depth) = args.depth {
        options.max_depth = Some(depth);
    }
    // Merge user exclusions with defaults.
    let user_excl = args.exclusions();
    for e in user_excl {
        options.exclusions.insert(e);
    }
    let root = scanner::scan_directory_with(&root_path, &options)
        .map_err(|e| format!("scan failed: {e}"))?;

    if args.stats {
        let stats = statistics::collect(&root);
        statistics::print_stats(&stats, renderer::use_color(args.no_color));
        return Ok(());
    }

    if args.interactive {
        use std::io::IsTerminal;
        if !std::io::stdout().is_terminal() {
            return Err("interactive mode requires a terminal".to_string());
        }
        let grouped_root = grouper::group_directory(root);
        return interactive::run(&grouped_root, renderer::use_color(args.no_color));
    }

    let project_name = root.name.clone();
    let graph = graph::build_graph(&root);

    let term_width = renderer::terminal_width(args.width);
    let preview_limit = if args.full {
        usize::MAX
    } else {
        args.preview.unwrap_or(3)
    };
    let layout = layout::compute_with(&graph, term_width, preview_limit);

    let use_color = renderer::use_color(args.no_color);

    println!();
    println!("  ◈ ARCHWIZ  /  {}  /  SOFTWARE MAP", project_name);
    println!(
        "  {}  ·  {} nodes  ·  {} connections",
        root_path.display(),
        graph.nodes.len(),
        graph.edges.len()
    );

    renderer::render(&graph, &layout, use_color);

    Ok(())
}
