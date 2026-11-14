# ArchWiz

### See the system behind your code.

ArchWiz is a Rust-based terminal architecture explorer that transforms project directory structures into visual flowcharts.

## Features

- **Visualisation** — Explore directory structures as terminal flowcharts.
- **Categorisation** — Identify source code, tests, configuration, documentation, and assets.
- **Logical Grouping** — Organise related files.
- **Structured Layout** — Generate balanced layouts with orthogonal connections.
- **Statistics** — View file counts, languages, directories, and storage usage.
- **Interactive Mode** — Navigate project structures directly in the terminal.
- **Gitignore Support** — Respect `.gitignore` rules.

## Installation

**Linux (x86_64 / aarch64)**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.sh | env INSTALLER_NO_MODIFY_PATH=1 sh
```

**Windows (PowerShell)**

```powershell
$env:INSTALLER_NO_MODIFY_PATH = "1"
irm https://github.com/abhaypratap08/ArchWiz/releases/latest/download/archwiz-installer.ps1 | iex
Remove-Item Env:INSTALLER_NO_MODIFY_PATH
```

Rust and Cargo are not required.

The default installation directory is `~/.local/bin` on Linux. Add it to your PATH if necessary:

```fish
fish_add_path "$HOME/.local/bin"
```

**Manual installation:** Download the appropriate archive from [GitHub Releases](https://github.com/abhaypratap08/ArchWiz/releases).

## Usage

```sh
archwiz                    # Analyse current directory
archwiz ~/Projects/App     # Analyse a project
archwiz --stats .          # Show statistics
archwiz --interactive .    # Interactive exploration
archwiz --help             # Show available options
```

## Scope

ArchWiz analyses filesystem structure, directory hierarchy, file categories, logical groupings, and project statistics.

It does not currently analyse source-code dependencies, imports, call graphs, or runtime relationships.

## Uninstallation

```sh
curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/uninstall.sh | sh
```

## Contributing

Issues and pull requests are welcome.

## License

[MIT License](LICENSE)

---

**ArchWiz — Map the structure. Trace the connections.**
