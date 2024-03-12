use std::{error::Error, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct YdnsConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

pub fn load_and_validate(config_file: &str) -> Result<YdnsConfig, Box<dyn Error>> {
    let config = load(config_file)?;
    if let Err(e) = validate(&config) {
        return Err(format!("Invalid configuration: {}", e).into());
    }

    Ok(config)
}

fn load(config_file: &str) -> Result<YdnsConfig, Box<dyn Error>> {
    let file_content = fs::read_to_string(&config_file)?;
    let config = serde_yaml::from_str::<YdnsConfig>(&file_content)?;

    Ok(config)
}

fn validate(config: &YdnsConfig) -> Result<(), Box<dyn Error>> {
    if config.base_url.is_empty() {
        return Err("Base URL is empty".into());
    }
    if config.username.is_empty() {
        return Err("Username is empty".into());
    }
    if config.password.is_empty() {
        return Err("Password is empty".into());
    }

    Ok(())
}
