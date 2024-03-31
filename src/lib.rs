use std::{error::Error, path::PathBuf};

use reqwest;

pub async fn get_current_ip(base_url: &str) -> Result<String, Box<dyn Error>> {
    match reqwest::get(format!("{}/ip", base_url)).await?.text().await {
        Ok(r) => Ok(r),
        Err(_) => {
            return Err("Couldn't find current IP".into());
        }
    }
}

pub async fn update_host(
    base_url: &str,
    username: &str,
    password: &str,
    host: &str,
    ip: &str,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    match client
        .get(format!("{}/update/?host={}&ip={}", base_url, host, ip))
        .basic_auth(username, Some(password))
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(r) => {
                if !r.contains("ok") || !r.contains("nochg") {
                    return Err(format!("Unexpected response: {}", r).into());
                }
                return Ok(r);
            }
            Err(e) => {
                return Err(format!("Couldn't parse response: {}", e).into());
            }
        },
        Err(e) => {
            return Err(format!("Something went terrible wrong! Error: {}", e).into());
        }
    }
}

pub fn get_ip_from_file(last_ip_file: &PathBuf) -> Result<String, Box<dyn Error>> {
    let ip = match std::fs::read_to_string(last_ip_file) {
        Ok(ip) => ip,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                // File does not exist, ignore error and return empty string
                return Ok("".to_string());
            }
            _ => {
                return Err(e.into());
            }
        },
    };
    Ok(ip)
}

pub fn write_ip_to_file(last_ip_file: &PathBuf, ip: &str) -> Result<(), Box<dyn Error>> {
    std::fs::write(last_ip_file, ip)?;
    Ok(())
}
