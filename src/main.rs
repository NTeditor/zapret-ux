mod config;

use anyhow::Result;
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

use crate::config::Config;

#[cfg(target_os = "android")]
const DEFAULT_CONFIG_PATH: &str = "/data/adb/zapret-ux/config.toml";

#[cfg(not(target_os = "android"))]
const DEFAULT_CONFIG_PATH: &str = "config.toml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE", default_value = DEFAULT_CONFIG_PATH)]
    config: Utf8PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start nfqws daemon
    Start,
    /// Stop nfqws daemon
    Stop,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Cli { config, command } = cli;
    let config: Config = confy::load_path(config)?;
    println!("{:#?}", config);
    Ok(())
}
