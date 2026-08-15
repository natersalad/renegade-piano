use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[cfg(test)]
mod tests;

#[derive(Debug, Parser)]
#[command(
    name = "renegade-piano",
    version,
    about = "Run and diagnose the Renegade Piano appliance"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Validate the application configuration.
    Check(ConfigArgs),

    /// Start the piano controller.
    Run(ConfigArgs),
}

#[derive(Debug, Args)]
pub struct ConfigArgs {
    /// Path to the application configuration.
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,
}
