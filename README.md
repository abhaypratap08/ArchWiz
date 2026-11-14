

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

<<<<<<< HEAD
```bash
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/install.sh | bash
=======
Prebuilt binaries are distributed through GitHub Releases. **Rust and Cargo are not required.**

**Linux**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.sh | env INSTALLER_NO_MODIFY_PATH=1 sh
>>>>>>> origin/fix/installer-path-safety
```

Install to a custom directory:

<<<<<<< HEAD
```bash
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/install.sh | ARCHWIZ_INSTALL_DIR="$HOME/Tools/ArchWiz" bash
```

The installer detects your Linux distribution, installs missing prerequisites, downloads the matching release, verifies its SHA-256 checksum, and installs ArchWiz.
=======
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
>>>>>>> origin/fix/installer-path-safety

**Windows**

<<<<<<< HEAD
Download the appropriate archive from [GitHub Releases](https://github.com/abhaypratap08/ArchWiz/releases), extract it, and run `archwiz.exe`.
=======
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
>>>>>>> origin/fix/installer-path-safety

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

