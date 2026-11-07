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
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.sh | env INSTALLER_NO_MODIFY_PATH=1 sh
```

**Windows (PowerShell)**

```powershell
$env:INSTALLER_NO_MODIFY_PATH = "1"
irm https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.ps1 | iex
Remove-Item Env:INSTALLER_NO_MODIFY_PATH
```

**Login and PATH safety:** The documented installer commands disable cargo-dist's automatic PATH modification. ArchWiz will not add commands to `~/.profile`, edit other shell startup files, or modify the Windows PATH registry. The configured install directory is `~/.local/bin` on Unix-like systems. If that directory is not already on your PATH, add it yourself to your interactive shell configuration.

For Fish, run:

```fish
fish_add_path "$HOME/.local/bin"
```

For Bash or Zsh, add the following line to `~/.bashrc` or `~/.zshrc`, respectively:

```sh
export PATH="$HOME/.local/bin:$PATH"
```

Then open a new terminal or reload the relevant shell configuration. Check the installation with `archwiz --version`.

> **Important:** These safety guarantees apply to the documented commands above. Running the raw release installer without `INSTALLER_NO_MODIFY_PATH=1` allows cargo-dist to manage PATH and may modify shell startup files.

**Existing installations:** This change prevents new installer runs from editing login files; it does not remove a line left by an older installation. If you installed an earlier ArchWiz release, inspect `~/.profile` for an old reference to `~/.config/archwiz/env.sh` or another ArchWiz environment helper. Remove only the stale ArchWiz line after checking it; do not delete or replace the whole profile.

### Custom Installation Directory

You can choose where `archwiz` is installed. The installer will still leave PATH and shell startup files unchanged; add the selected directory to PATH yourself if necessary.

**Linux**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.sh | env INSTALLER_NO_MODIFY_PATH=1 ARCHWIZ_INSTALL_DIR="$HOME/Tools/ArchWiz" sh
```

**Windows (PowerShell)**

```powershell
$env:INSTALLER_NO_MODIFY_PATH = "1"
$env:ARCHWIZ_INSTALL_DIR = "$HOME\Tools\ArchWiz"
irm https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.ps1 | iex
Remove-Item Env:INSTALLER_NO_MODIFY_PATH
Remove-Item Env:ARCHWIZ_INSTALL_DIR
```

If you choose a custom directory, add that directory to the PATH configuration for your interactive shell (Fish: `fish_add_path "$HOME/Tools/ArchWiz"`; Bash/Zsh: add `export PATH="$HOME/Tools/ArchWiz:$PATH"` to `~/.bashrc` or `~/.zshrc`). Do not add an unguarded reference to an ArchWiz helper script in `~/.profile`.

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
