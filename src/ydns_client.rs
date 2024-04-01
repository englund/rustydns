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
                if !r.contains("ok") && !r.contains("nochg") {
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

#[cfg(test)]
mod tests {
    use base64::prelude::*;

    use super::*;

    #[tokio::test]
    async fn test_get_current_ip() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let _mock = server
            .mock("GET", "/ip")
            .with_status(200)
            .with_body("1.3.3.7")
            .create();

        let ip = get_current_ip(&url).await.unwrap();
        assert_eq!(ip, "1.3.3.7");
    }

    #[tokio::test]
    async fn test_update_host() {
        let username = "test";
        let password = "test";
        let auth_header = BASE64_STANDARD.encode(format!("{}:{}", username, password));
        let host = "example.com";
        let ip = "1.3.3.7";

        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let _mock = server
            .mock("GET", format!("/update/?host={}&ip={}", host, ip).as_str())
            .match_header("Authorization", format!("Basic {}", auth_header).as_str())
            .with_status(200)
            .with_body("ok")
            .create();

        let response = update_host(&url, &username, &password, &host, &ip)
            .await
            .unwrap();

        assert_eq!(response, "ok");
    }
}
