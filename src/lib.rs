use std::error::Error;

use reqwest;

const YDNS_BASE_URL: &str = "https://ydns.io/api/v1";

pub async fn get_current_ip() -> Result<String, Box<dyn Error>> {
    match reqwest::get(format!("{YDNS_BASE_URL}/ip"))
        .await?
        .text()
        .await
    {
        Ok(r) => Ok(r),
        Err(_) => {
            return Err("Couldn't find current IP".into());
        }
    }
}

pub async fn update_host(
    username: &str,
    password: &str,
    host: &str,
    ip: &str,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    match client
        .get(format!("{YDNS_BASE_URL}/update/?host={host}&ip={ip}"))
        .basic_auth(username, Some(password))
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(r) => {
                if !r.contains("ok") {
                    return Err(format!("Something went wrong updating the host: {}", r).into());
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
