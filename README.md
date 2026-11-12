# ArchWiz

### See the system behind your code

**ArchWiz** is a terminal-native architecture explorer written in Rust. It transforms a project's filesystem structure into a visual architectural flowchart, helping you understand how a codebase is organised.

Unlike traditional tree listings that simply display files and directories, ArchWiz presents the structure as a navigable visual map.

## Features

- **Visual Architecture** — Transform filesystem structures into terminal-native flowcharts.
- **Smart Categorisation** — Identify and distinguish source code, tests, configuration, documentation, and assets.
- **Logical Grouping** — Group related files to make large directory structures easier to understand.
- **Structured Layout** — Balanced node placement with orthogonal connections for clearer visualisation.
- **Project Statistics** — Explore file counts, directories, languages, storage usage, and other structural information.
- **Interactive Exploration** — Navigate the generated architecture directly from your terminal.
- **Respect for Project Structure** — Follow `.gitignore` rules and exclude common generated files and directories.

## Installation

```bash
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/install.sh | bash
```

The installer detects Linux x86_64 or aarch64, installs missing prerequisites using your distribution's package manager, downloads the matching release, verifies its SHA-256 checksum, and installs `archwiz` to `$HOME/.local/bin` by default.

To install into a different directory:

```bash
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/install.sh | ARCHWIZ_INSTALL_DIR="$HOME/Tools/ArchWiz" bash
```

If the installation directory is not on your `PATH`, add it to your shell configuration manually.

### Windows

Windows archives are published on the [GitHub Releases](https://github.com/abhaypratap08/ArchWiz/releases) page. Download the archive matching your architecture (`x86_64-pc-windows-msvc` or `aarch64-pc-windows-msvc`), extract it, and run `archwiz.exe`. The Linux `install.sh` does not support native Windows.

To uninstall:

```bash
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/uninstall.sh | bash
```

## Usage

Run `archwiz` in the current directory:

```sh
archwiz
```

Explore a specific project:

```sh
archwiz ~/Projects/YourProject
```

Display project statistics:

```sh
archwiz --stats .
```

Launch interactive mode:

```sh
archwiz --interactive .
```

View available options:

```sh
archwiz --help
```

## What ArchWiz Analyses

ArchWiz focuses on **filesystem architecture**:

- Directory hierarchy
- File organisation
- File categories
- Logical file groupings
- Project-level structural statistics

It does **not** currently analyse source-code imports, dependencies, call graphs, or runtime relationships. Its visual connections represent filesystem organisation and grouping, not code-level dependencies.

## Project Status

ArchWiz is an ongoing project built as a hands-on journey into Rust, filesystem traversal, graph algorithms, terminal rendering, and visualisation.

The goal is to make understanding a project's structure easier without leaving the terminal.

## Contributing

Contributions, ideas, and feedback are welcome. Feel free to open an issue or submit a pull request.

## License

Licensed under the [MIT License](LICENSE).

---

**ArchWiz — MAP THE STRUCTURE. TRACE THE CONNECTIONS.**
