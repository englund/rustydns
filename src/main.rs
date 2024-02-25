use clap::Parser;
use log::error;
use log::info;
use std::process::exit;

mod config;
mod logging;

use ydns_updater::{get_current_ip, update_host};

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(required = true, long, short = 'H')]
    host: Vec<String>,

    #[arg(long, short, default_value = "ydns.yaml")]
    config: String,

    #[arg(long, short, default_value = "")]
    logfile: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    logging::setup(&args.logfile);

    let config = match config::setup(&args.config) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "Couldn't read the configuration file {}: {}",
                &args.config, e
            );
            exit(1)
        }
    };

    if let Err(e) = config::validate(&config) {
        error!("Invalid configuration: {}", e);
        exit(1)
    }

    let current_ip = get_current_ip().await.unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    });

    info!("Current IP: {current_ip}");

    for host in args.host.iter() {
        info!("Host: {host}");
        if let Err(e) = update_host(&config.username, &config.password, &host, &current_ip).await {
            error!("Could not update host {}: {}", host, e);
            exit(1)
        }
    }
}
