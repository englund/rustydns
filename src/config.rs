use std::{error::Error, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct YdnsConfig {
    pub username: String,
    pub password: String,
}

pub fn setup(config_file: &str) -> Result<YdnsConfig, Box<dyn Error>> {
    let file_content = fs::read_to_string(&config_file)?;
    let config = serde_yaml::from_str::<YdnsConfig>(&file_content)?;

    Ok(config)
}

pub fn validate(config: &YdnsConfig) -> Result<(), Box<dyn Error>> {
    if config.username.is_empty() {
        return Err("Username is empty".into());
    }
    if config.password.is_empty() {
        return Err("Password is empty".into());
    }

    Ok(())
}
