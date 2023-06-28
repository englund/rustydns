use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};
use std::{fs, process::exit};

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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let file_content = match fs::read_to_string(&args.file) {
        Ok(f) => f,
        Err(e) => {
            println!("Couldn't read the configuration file {}: {}", &args.file, e);
            exit(1)
        }
    };

    let config = match serde_yaml::from_str::<Config>(&file_content) {
        Ok(c) => c,
        Err(e) => {
            println!(
                "Couldn't parse the configuration file {}: {}",
                &args.file, e
            );
            exit(1)
        }
    };

    if config.username.is_empty() {
        println!("The username needs to be configured");
        exit(1)
    }

    if config.password.is_empty() {
        println!("The password needs to be configured");
        exit(1)
    }

    let current_ip = match get_current_ip().await {
        Ok(r) => r,
        Err(_) => {
            println!("Couldn't find current IP");
            exit(1)
        }
    };
    println!("Current IP: {current_ip}");

    for arg in args.host.iter() {
        println!("Host: {arg}");
        match update_host(&config.username, &config.password, &arg, &current_ip).await {
            Ok(response) => match response.text().await {
                Ok(r) => println!("Result: {}", r),
                Err(e) => {
                    println!("Could not parse response: {}", e);
                    exit(1)
                }
            },
            Err(e) => {
                println!("Something went terrible wrong! Error: {}", e);
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
