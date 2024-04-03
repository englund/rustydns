use std::error::Error;

use confique::Config;

#[derive(Config)]
pub(crate) struct YdnsConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

pub(crate) fn load(config_file: &str) -> Result<YdnsConfig, Box<dyn Error>> {
    let config = Config::builder()
        .file(config_file)
        .file("/etc/ydns/ydns.yaml")
        .load()?;

    Ok(config)
}
