use reqwest;

const YDNS_BASE_URL: &str = "https://ydns.io/api/v1";

pub async fn get_current_ip() -> Result<String, reqwest::Error> {
    reqwest::get(format!("{YDNS_BASE_URL}/ip"))
        .await?
        .text()
        .await
}

pub async fn update_host(
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
