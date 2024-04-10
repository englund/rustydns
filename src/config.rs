use std::{error::Error, path::PathBuf};

use confique::Config;

#[derive(Config)]
pub(crate) struct YdnsConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub hosts: Vec<String>,
    pub wait_time: u64,
}

pub(crate) fn load(config_file: &Vec<PathBuf>) -> Result<YdnsConfig, Box<dyn Error>> {
    let mut builder = Config::builder();
    for file in config_file {
        builder = builder.file(file);
    }

    Ok(builder.load()?)
}
