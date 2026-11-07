// Central theme tokens (three layers: primitive -> semantic -> component).

// Primitive ANSI 256 color indices.
pub const COLOR_CYAN_ICY: u8 = 51;
pub const COLOR_TEAL: u8 = 43;
pub const COLOR_BLUE_CLEAR: u8 = 75;
pub const COLOR_AMBER_MUTED: u8 = 214;
pub const COLOR_BLUE_GRAY: u8 = 109;
pub const COLOR_VIOLET: u8 = 141;
pub const COLOR_GREEN_SUBDUED: u8 = 114;
pub const COLOR_GRAY_BLUE: u8 = 102;
pub const COLOR_MUTED_CYAN: u8 = 73;

// Semantic color roles.
pub fn project_color() -> u8 {
    COLOR_CYAN_ICY
}
pub fn directory_color() -> u8 {
    COLOR_TEAL
}
pub fn source_color() -> u8 {
    COLOR_BLUE_CLEAR
}
pub fn config_color() -> u8 {
    COLOR_AMBER_MUTED
}
pub fn docs_color() -> u8 {
    COLOR_BLUE_GRAY
}
pub fn test_color() -> u8 {
    COLOR_VIOLET
}
pub fn asset_color() -> u8 {
    COLOR_GREEN_SUBDUED
}
pub fn other_color() -> u8 {
    COLOR_GRAY_BLUE
}
pub fn generated_color() -> u8 {
    240
}
pub fn connection_color() -> u8 {
    COLOR_MUTED_CYAN
}
pub fn metadata_color() -> u8 {
    COLOR_GRAY_BLUE
}

pub fn kind_color(kind: &crate::graph::GraphNodeKind) -> u8 {
    use crate::classifier::FileKind;
    use crate::graph::GraphNodeKind;
    match kind {
        GraphNodeKind::Project => project_color(),
        GraphNodeKind::Directory => directory_color(),
        GraphNodeKind::Group => directory_color(),
        GraphNodeKind::File(fk) => match fk {
            FileKind::Source => source_color(),
            FileKind::Test => test_color(),
            FileKind::Configuration => config_color(),
            FileKind::Documentation => docs_color(),
            FileKind::Asset => asset_color(),
            FileKind::Generated => generated_color(),
            FileKind::Other => other_color(),
        },
    }
}
