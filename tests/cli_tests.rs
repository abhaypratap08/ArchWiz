use archwiz::cli::Cli;
use clap::Parser;

#[test]
fn default_no_path() {
    let cli = Cli::try_parse_from(["archwiz"]).unwrap();
    assert!(cli.path.is_none());
    assert!(!cli.no_color);
}

#[test]
fn explicit_path() {
    let cli = Cli::try_parse_from(["archwiz", "/tmp/foo"]).unwrap();
    assert_eq!(cli.path.unwrap().to_str().unwrap(), "/tmp/foo");
}

#[test]
fn depth_and_exclude() {
    let cli = Cli::try_parse_from([
        "archwiz",
        "--depth",
        "3",
        "--exclude",
        "target,node_modules",
    ])
    .unwrap();
    assert_eq!(cli.depth, Some(3));
    let excl = cli.exclusions();
    assert!(excl.contains("target"));
    assert!(excl.contains("node_modules"));
}

#[test]
fn no_color_flag() {
    let cli = Cli::try_parse_from(["archwiz", "--no-color"]).unwrap();
    assert!(cli.no_color);
}

#[test]
fn width_override() {
    let cli = Cli::try_parse_from(["archwiz", "--width", "120"]).unwrap();
    assert_eq!(cli.width, Some(120));
}

#[test]
fn help_works() {
    let res = Cli::try_parse_from(["archwiz", "--help"]);
    assert!(res.is_err()); // clap exits with error code for help
    let err = res.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
}

#[test]
fn version_works() {
    let res = Cli::try_parse_from(["archwiz", "--version"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
}

#[test]
fn invalid_arg() {
    let res = Cli::try_parse_from(["archwiz", "--bogus"]);
    assert!(res.is_err());
}
