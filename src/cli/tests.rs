use super::{Cli, Commands};
use clap::Parser;
use std::path::PathBuf;

#[test]
fn check_uses_default_config_path() {
    let cli = Cli::try_parse_from(["renegade-piano", "check"]).expect("check command should parse");

    let Commands::Check(args) = cli.command else {
        panic!("expected check command");
    };

    assert_eq!(args.config, PathBuf::from("config.toml"));
}

#[test]
fn run_accepts_custom_config_path() {
    let cli = Cli::try_parse_from([
        "renegade-piano",
        "run",
        "--config",
        "/etc/renegade-piano/config.toml",
    ])
    .expect("run command should parse");

    let Commands::Run(args) = cli.command else {
        panic!("expected run command");
    };

    assert_eq!(
        args.config,
        PathBuf::from("/etc/renegade-piano/config.toml")
    );
}

#[test]
fn command_is_required() {
    let error =
        Cli::try_parse_from(["renegade-piano"]).expect_err("missing command should be rejected");

    assert_eq!(error.exit_code(), 2);
}
