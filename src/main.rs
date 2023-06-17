use reqwest;
use std::{env, process::exit};

const YDNS_BASE_URL: &str = "https://ydns.io/api/v1";

#[tokio::main]
async fn main() {
    let ydns_username = read_env_or_exit("YDNS_USERNAME");
    let ydns_password = read_env_or_exit("YDNS_PASSWORD");

    let current_ip = get_current_ip().await.unwrap();
    println!("Current IP: {current_ip}");

    let args = env::args();
    for arg in args.skip(1) {
        println!("Host: {arg}");
        let response = update_host(&ydns_username, &ydns_password, &arg, &current_ip).await;
        match response {
            Ok(response) => {
                let result = response.text().await.unwrap();
                println!("Result: {result}")
            }
            Err(_) => panic!("Something went terrible wrong"),
        }
    }
}

fn read_env_or_exit(name: &str) -> String {
    match env::var(name) {
        Ok(env) => env,
        Err(_) => {
            println!("Environment variable {name} doesn't exist");
            exit(1)
        }
    }
}

async fn get_current_ip() -> Result<String, reqwest::Error> {
    reqwest::get(format!("{YDNS_BASE_URL}/ip"))
        .await
        .unwrap()
        .text()
        .await
}

async fn update_host(
    username: &String,
    password: &String,
    host: &String,
    ip: &String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get(format!("{YDNS_BASE_URL}/update/?host={host}&ip={ip}"))
        .basic_auth(username, Some(password))
        .send()
        .await
}
