mod config;

use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());

    let config = Config::load(path)?;

    println!("{config:#?}");

    Ok(())
}
