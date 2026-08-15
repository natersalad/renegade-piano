mod cli;
mod config;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => {
            Config::load(&args.config)?;
            println!("configuration is valid: {}", args.config.display());
        }
        Commands::Run(args) => {
            let config = Config::load(args.config)?;

            println!("{config:#?}");
        }
    }

    Ok(())
}
