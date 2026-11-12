

# ArchWiz

### See the system behind your code.

ArchWiz is a terminal-native architecture explorer written in Rust. It transforms a project's filesystem structure into a visual flowchart, making codebase organisation easier to understand.

## Features

- **Visualisation** — Explore directory structures as terminal flowcharts.
- **Smart Categorisation** — Identify source code, tests, configuration, documentation, and assets.
- **Logical Grouping** — Group related files for easier navigation.
- **Structured Layout** — Balanced node placement with orthogonal connections.
- **Project Statistics** — View file counts, directories, languages, and storage usage.
- **Interactive Mode** — Navigate project structures directly in the terminal.
- **Gitignore Support** — Respect `.gitignore` rules and exclude common generated files.

## Installation

**Linux (x86_64 / aarch64)**

```bash
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/install.sh | bash
```

Install to a custom directory:

```bash
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/install.sh | ARCHWIZ_INSTALL_DIR="$HOME/Tools/ArchWiz" bash
```

The installer detects your Linux distribution, installs missing prerequisites, downloads the matching release, verifies its SHA-256 checksum, and installs ArchWiz.

**Windows**

Download the appropriate archive from [GitHub Releases](https://github.com/abhaypratap08/ArchWiz/releases), extract it, and run `archwiz.exe`.

## Usage

```bash
# Analyse the current directory
archwiz

# Analyse a specific project
archwiz ~/Projects/YourProject

# Display project statistics
archwiz --stats .

# Interactive exploration
archwiz --interactive .

# View available options
archwiz --help
```

## Scope

ArchWiz analyses filesystem structure, including:

- Directory hierarchy
- File organisation and categories
- Logical file groupings
- Project-level statistics

It does not currently analyse source-code imports, dependencies, call graphs, or runtime relationships. Visual connections represent filesystem organisation, not code dependencies.

## Uninstallation

```bash
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/uninstall.sh | bash
```

## Contributing

Issues, ideas, and pull requests are welcome.

## License

[MIT License](LICENSE)

---

**ArchWiz — Map the structure. Trace the connections.**

