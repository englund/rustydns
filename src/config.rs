use std::{error::Error, path::PathBuf};

use confique::Config;

#[derive(Config)]
pub(crate) struct YdnsConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub hosts: Vec<String>,

    #[config(default = 900)]
    pub wait_time: u64,

    #[config(default = false)]
    pub daemon: bool,
}

pub(crate) fn load(config_file: &Vec<PathBuf>) -> Result<YdnsConfig, Box<dyn Error>> {
    let mut builder = Config::builder();
    for file in config_file {
        builder = builder.file(file);
    }

    Ok(builder.load()?)
}
