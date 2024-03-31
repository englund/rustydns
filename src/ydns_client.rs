use std::error::Error;

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
