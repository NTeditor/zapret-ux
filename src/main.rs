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

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn main() {
    panic!("Your OS is not android or linux");
}

#[cfg(any(target_os = "android", target_os = "linux"))]
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
            println!("Starting daemon");
            iptables.setup_rules(ports)?;
            nfqws.run(opt)?;
        }
        Commands::Stop => {
            println!("Stoping daemon");
            iptables.clean_rules()?;
            nfqws.kill()?;
        }
        Commands::Restart => {
            println!("Restarting daemon");
            iptables.clean_rules()?;
            nfqws.kill()?;
            iptables.setup_rules(ports)?;
            nfqws.run(opt)?;
        }
        Commands::Status => {
            if nfqws.is_running()? {
                println!("Daemon is running");
            } else {
                println!("Daemon is not running");
            }
        }
        Commands::Autostart => {
            println!("Зачем выпускать HL3 сегодня, когда есть завтра?");
            if autostart_enabled {
                iptables.setup_rules(ports)?;
                nfqws.run(opt)?;
            }
        }
    }
    Ok(())
}

fn init_logger() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        #[cfg(debug_assertions)]
        return EnvFilter::new("info");
        #[cfg(not(debug_assertions))]
        return EnvFilter::new("warn");
    });
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
