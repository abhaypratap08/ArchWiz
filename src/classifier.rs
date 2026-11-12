use crate::model::FileNode;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileKind {
    Source,
    Test,
    Configuration,
    Documentation,
    Asset,
    Generated,
    Other,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLanguage {
    Rust,
    Java,
    JavaScript,
    TypeScript,
    Python,
    Go,
    C,
    Cpp,
    CSharp,
    Php,
    Ruby,
    Kotlin,
    Swift,
    Shell,
    Other,
}

pub fn classify_file(file: &FileNode) -> FileKind {
    classify_path(&file.path)
}

pub fn classify_path(path: &Path) -> FileKind {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let in_test_dir = path.components().any(|component| {
        matches!(
            component.as_os_str().to_str(),
            Some("test" | "tests" | "__tests__" | "spec" | "specs")
        )
    });

    let is_test_file = name.starts_with("test_")
        || name.starts_with("test.")
        || name.ends_with("_test.rs")
        || name.ends_with("_test.go")
        || name.ends_with("_test.py")
        || name.ends_with(".test.js")
        || name.ends_with(".test.jsx")
        || name.ends_with(".test.ts")
        || name.ends_with(".test.tsx")
        || name.ends_with(".spec.js")
        || name.ends_with(".spec.jsx")
        || name.ends_with(".spec.ts")
        || name.ends_with(".spec.tsx");

    if in_test_dir || is_test_file {
        return FileKind::Test;
    }

    if name.starts_with("readme")
        || name == "license"
        || name == "license.md"
        || name == "changelog"
        || name == "changelog.md"
        || matches!(extension.as_str(), "md" | "rst" | "adoc" | "txt")
    {
        return FileKind::Documentation;
    }

    if is_configuration_name(&name) || is_configuration_ext(&extension) {
        return FileKind::Configuration;
    }

    if is_asset_ext(&extension) {
        return FileKind::Asset;
    }

    if is_generated_ext(&extension) {
        return FileKind::Generated;
    }

    if is_source_ext(&extension) {
        return FileKind::Source;
    }

    FileKind::Other
}

fn is_configuration_name(name: &str) -> bool {
    matches!(
        name,
        "cargo.toml"
            | "cargo.lock"
            | "package.json"
            | "package-lock.json"
            | "pnpm-lock.yaml"
            | "yarn.lock"
            | ".gitignore"
            | "dockerfile"
            | "makefile"
            | ".env"
            | "cmakelists.txt"
            | "go.mod"
            | "go.sum"
            | "pyproject.toml"
            | "requirements.txt"
            | "setup.py"
            | "setup.cfg"
            | "tsconfig.json"
            | "vite.config.js"
            | "vite.config.ts"
    ) || name.starts_with(".env")
}

fn is_configuration_ext(ext: &str) -> bool {
    matches!(
        ext,
        "toml" | "yaml" | "yml" | "json" | "ini" | "cfg" | "conf" | "xml" | "lock"
    )
}

fn is_asset_ext(ext: &str) -> bool {
    matches!(
        ext,
        "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "ico"
            | "webp"
            | "svg"
            | "mp3"
            | "mp4"
            | "wav"
            | "woff"
            | "woff2"
            | "ttf"
            | "otf"
            | "eot"
    )
}

fn is_generated_ext(ext: &str) -> bool {
    matches!(
        ext,
        "pyc" | "pyo" | "class" | "o" | "obj" | "so" | "dll" | "exe" | "log"
    )
}

fn is_source_ext(ext: &str) -> bool {
    matches!(
        ext,
        "rs" | "java"
            | "js"
            | "jsx"
            | "mjs"
            | "cjs"
            | "ts"
            | "tsx"
            | "py"
            | "go"
            | "c"
            | "cpp"
            | "cc"
            | "cxx"
            | "h"
            | "hpp"
            | "cs"
            | "php"
            | "rb"
            | "swift"
            | "kt"
            | "kts"
            | "scala"
            | "sh"
            | "bash"
            | "zsh"
            | "fish"
            | "sql"
            | "lua"
            | "r"
            | "dart"
            | "ex"
            | "exs"
            | "zig"
            | "nim"
    )
}

pub fn file_kind_label(kind: FileKind) -> &'static str {
    match kind {
        FileKind::Source => "SOURCE",
        FileKind::Test => "TEST",
        FileKind::Configuration => "CONFIG",
        FileKind::Documentation => "DOCS",
        FileKind::Asset => "ASSET",
        FileKind::Generated => "GEN",
        FileKind::Other => "OTHER",
    }
}
#[allow(dead_code)]
pub fn source_language(path: &Path) -> SourceLanguage {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "rs" => SourceLanguage::Rust,
        "java" => SourceLanguage::Java,
        "js" | "jsx" | "mjs" | "cjs" => SourceLanguage::JavaScript,
        "ts" | "tsx" => SourceLanguage::TypeScript,
        "py" => SourceLanguage::Python,
        "go" => SourceLanguage::Go,
        "c" | "h" => SourceLanguage::C,
        "cpp" | "cc" | "cxx" | "hpp" => SourceLanguage::Cpp,
        "cs" => SourceLanguage::CSharp,
        "php" => SourceLanguage::Php,
        "rb" => SourceLanguage::Ruby,
        "kt" | "kts" => SourceLanguage::Kotlin,
        "swift" => SourceLanguage::Swift,
        "sh" | "bash" | "zsh" | "fish" => SourceLanguage::Shell,
        _ => SourceLanguage::Other,
    }
}
