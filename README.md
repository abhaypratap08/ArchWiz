# ARCHWIZ

ARCHWIZ is a terminal-native software architecture explorer. It scans a project and renders its structure as a connected flowchart directly in the terminal.

## Installation from source

```fish
cargo install --path .
```

## Usage

```fish
archwiz
archwiz .
archwiz ~/Projects/Bravo6
archwiz --depth 3
archwiz --exclude target,node_modules
archwiz --no-color
archwiz --help
archwiz --version
```

## Options

- `-h, --help` Print help
- `-V, --version` Print version
- `-d, --depth <N>` Limit directory depth
- `-e, --exclude <PATTERNS>` Exclude directories (comma separated)
- `--no-color` Disable terminal colors
- `--width <COLUMNS>` Override terminal width

## Pipeline

```text
Filesystem
    ↓
Scanner
    ↓
Project Model
    ↓
Classifier
    ↓
Graph Generator
    ↓
Layout Engine
    ↓
Terminal Renderer
```

## Supported classifications

Source, Test, Configuration, Documentation, Asset, Other.

Source languages: Rust, Java, JavaScript, TypeScript, Python, Go, C, C++, C#, PHP, Ruby, Kotlin, Swift, and more.

## Exclusion rules

Default excluded directories: `.git`, `target`, `node_modules`, `dist`, `build`. Add more with `--exclude`.

## Limitations

- Directory membership is shown, not source-level dependency analysis.
- Symlinks are not followed.
- Interactive navigation is not implemented.

## Development

```fish
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```