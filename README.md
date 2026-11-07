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

### Binary Installation (Recommended)

Prebuilt binaries are distributed through GitHub Releases. **Rust and Cargo are not required.**

**Linux**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.sh | sh
```

**Windows (PowerShell)**

```powershell
irm https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.ps1 | iex
```

> **Note:** Binary installers will become available after the first GitHub Release is published.

### Custom Installation Directory

You can choose where `archwiz` is installed.

**Linux**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.sh | ARCHWIZ_INSTALL_DIR="$HOME/Tools/ArchWiz" sh
```

**Windows (PowerShell)**

```powershell
$env:ARCHWIZ_INSTALL_DIR = "$HOME\Tools\ArchWiz"
irm https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.ps1 | iex
```

Ensure your installation directory is included in your system's `PATH`.

### Manual Installation

1. Visit [GitHub Releases](https://github.com/abhaypratap08/ArchWiz/releases).
2. Download the archive matching your operating system and architecture.
3. Extract the archive.
4. Move the `archwiz` executable (or `archwiz.exe` on Windows) into a directory included in your `PATH`.

### Install from Source

If you have Rust and Cargo installed:

```sh
cargo install --git https://github.com/abhaypratap08/ArchWiz
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
