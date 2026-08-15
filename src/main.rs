mod cli;
mod config;
mod system;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use std::process::ExitCode;
use system::SystemReport;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => {
            Config::load(&args.config)?;
            println!("✓ configuration: {}", args.config.display());

            let report = SystemReport::inspect();
            report.print();
            report.ensure_ready()?;
        }
        Commands::Run(args) => {
            let config = Config::load(args.config)?;

            println!("{config:#?}");
        }
    }

    Ok(())
}
