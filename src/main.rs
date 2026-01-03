mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::*;
use iptables::{FirewallProvider, Iptables};
use nfqws::{BypassSoftware, Nfqws};
use std::path::PathBuf;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::ChronoUtc},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[cfg(target_os = "android")]
const DEFAULT_CONFIG_PATH: &str = "/data/adb/zapret-ux/config.toml";
#[cfg(not(target_os = "android"))]
const DEFAULT_CONFIG_PATH: &str = ".config.toml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to config file
    #[arg(short, long, value_name = "FILE", default_value = DEFAULT_CONFIG_PATH)]
    config: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start daemon
    Start,
    /// Stop daemon
    Stop,
    /// Restart daemon
    Restart,
    /// Print status daemon
    Status,
    #[command(hide = true)]
    Autostart,
}

fn main() -> Result<()> {
    init_logger();
    let cli = Cli::parse();
    let config: Config = confy::load_path(cli.config)?;
    let Config {
        iptables,
        nfqws,
        mark_supported,
        autostart_enabled,
    } = config;

    let ConfigIptables {
        iptables_path,
        connbytes_supported,
        ports,
    } = iptables;

    let ConfigNfqws {
        nfqws_path,
        pgrep_path,
        pkill_path,
        filter_mode,
        opt,
    } = nfqws;

    let iptables = Iptables::new(iptables_path, mark_supported, connbytes_supported);
    let nfqws = Nfqws::new(
        nfqws_path,
        pgrep_path,
        pkill_path,
        mark_supported,
        filter_mode,
    );
    match cli.command {
        Commands::Start => {
            println!("start");
            iptables.setup_rules(ports)?;
            nfqws.run(opt)?;
        }
        Commands::Stop => {
            println!("stop");
            iptables.clean_rules()?;
            nfqws.kill()?;
        }
        Commands::Restart => {
            println!("restart");
            iptables.clean_rules()?;
            nfqws.kill()?;
            iptables.setup_rules(ports)?;
            nfqws.run(opt)?;
        }
        Commands::Status => {
            println!("status");
            if nfqws.is_running()? {
                println!("Daemon is running");
            } else {
                println!("Daemon is not running");
            }
        }
        Commands::Autostart => {
            println!("autostart");
            if autostart_enabled {
                iptables.setup_rules(ports)?;
                nfqws.run(opt)?;
            }
        }
    }
    Ok(())
}

fn init_logger() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let timer = ChronoUtc::new("%H:%M:%S".to_string());

    let fmt_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_target(true)
        .with_timer(timer)
        .compact();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Logger is initialized");
}
