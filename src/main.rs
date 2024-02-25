use clap::Parser;
use log::{error, info, LevelFilter};
use serde::{Deserialize, Serialize};
use simplelog::{
    ColorChoice, CombinedLogger, Config as SimplelogConfig, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};
use std::{
    fs::{self, OpenOptions},
    process::exit,
};

use ydns_updater::{get_current_ip, update_host};

#[derive(Serialize, Deserialize, Debug)]
struct YdnsConfig {
    username: String,
    password: String,
}

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

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        LevelFilter::Info,
        SimplelogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];
    if !args.logfile.is_empty() {
        let logfile = WriteLogger::new(
            LevelFilter::Warn,
            SimplelogConfig::default(),
            OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(&args.logfile)
                .unwrap(),
        );
        loggers.push(logfile);
    }
    CombinedLogger::init(loggers).unwrap();

    let file_content = match fs::read_to_string(&args.config) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "Couldn't read the configuration file {}: {}",
                &args.config, e
            );
            exit(1)
        }
    };

    let config = match serde_yaml::from_str::<YdnsConfig>(&file_content) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "Couldn't parse the configuration file {}: {}",
                &args.config, e
            );
            exit(1)
        }
    };

    if config.username.is_empty() {
        error!("The username needs to be configured");
        exit(1)
    }

    if config.password.is_empty() {
        error!("The password needs to be configured");
        exit(1)
    }

    let current_ip = match get_current_ip().await {
        Ok(r) => r,
        Err(_) => {
            error!("Couldn't find current IP");
            exit(1)
        }
    };

    info!("Current IP: {current_ip}");

    for arg in args.host.iter() {
        info!("Host: {arg}");
        match update_host(&config.username, &config.password, &arg, &current_ip).await {
            Ok(response) => match response.text().await {
                Ok(r) => {
                    if !r.contains("ok") {
                        error!("Something went wrong updating the host: {}", r);
                        exit(1)
                    }
                }
                Err(e) => {
                    error!("Couldn't parse response: {}", e);
                    exit(1)
                }
            },
            Err(e) => {
                error!("Something went terrible wrong! Error: {}", e);
                exit(1)
            }
        }
    }
}
