use clap::{Parser, Subcommand};
use log::{error, info};
use std::path::PathBuf;
use std::process::exit;

use ydns_updater::{get_current_ip, update_host};

mod config;
mod logging;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Get current ip.
    Ip,

    /// Update host(s) with current ip.
    Update {
        /// The host(s) to update
        #[arg(required = true, long, short = 'H')]
        host: Vec<String>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct GlobalOpts {
    /// The configuration file
    #[arg(long, short, default_value = "ydns.yaml")]
    config: String,

    /// Optional log file
    #[arg(long, short)]
    logfile: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = App::parse();

    logging::setup(args.global_opts.logfile);

    let config = match config::setup(&args.global_opts.config) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "Could not read the configuration file {}: {}",
                &args.global_opts.config, e
            );
            exit(1)
        }
    };

    match args.command {
        Command::Ip => get_ip(&config).await,
        Command::Update { host } => update(&config, host).await,
    }

    if let Err(e) = config::validate(&config) {
        error!("Invalid configuration: {}", e);
        exit(1)
    }
}

async fn get_ip(config: &config::YdnsConfig) {
    match get_current_ip(&config.base_url).await {
        Ok(ip) => {
            info!("Current IP: {ip}");
        }
        Err(e) => {
            error!("Could not get current IP: {}", e);
            exit(1)
        }
    };
}

async fn update(config: &config::YdnsConfig, host: Vec<String>) {
    let current_ip = match get_current_ip(&config.base_url).await {
        Ok(ip) => ip,
        Err(e) => {
            error!("Could not get current IP: {}", e);
            exit(1)
        }
    };

    info!("Current IP: {current_ip}");

    for host in host.iter() {
        info!("Host: {host}");
        if let Err(e) = update_host(
            &config.base_url,
            &config.username,
            &config.password,
            &host,
            &current_ip,
        )
        .await
        {
            error!("Could not update host {}: {}", host, e);
            exit(1)
        }
    }
}
