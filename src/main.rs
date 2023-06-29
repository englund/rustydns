use clap::Parser;
use log::{error, info, LevelFilter};
use reqwest;
use serde::{Deserialize, Serialize};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};
use std::{
    fs::{self, File},
    process::exit,
};

const YDNS_BASE_URL: &str = "https://ydns.io/api/v1";

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    username: String,
    password: String,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, default_value = "ydns.yaml")]
    file: String,

    #[arg(required = true, long, short = 'H')]
    host: Vec<String>,

    #[arg(long, short, default_value = "ydns-updater.log")]
    logfile: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Warn,
            simplelog::Config::default(),
            File::create(&args.logfile).unwrap(),
        ),
    ])
    .unwrap();

    let file_content = match fs::read_to_string(&args.file) {
        Ok(f) => f,
        Err(e) => {
            error!("Couldn't read the configuration file {}: {}", &args.file, e);
            exit(1)
        }
    };

    let config = match serde_yaml::from_str::<Config>(&file_content) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "Couldn't parse the configuration file {}: {}",
                &args.file, e
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

async fn get_current_ip() -> Result<String, reqwest::Error> {
    reqwest::get(format!("{YDNS_BASE_URL}/ip"))
        .await?
        .text()
        .await
}

async fn update_host(
    username: &str,
    password: &str,
    host: &str,
    ip: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get(format!("{YDNS_BASE_URL}/update/?host={host}&ip={ip}"))
        .basic_auth(username, Some(password))
        .send()
        .await
}
