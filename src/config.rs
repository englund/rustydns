use std::{error::Error, path::PathBuf};

use confique::Config;

#[derive(Config)]
pub(crate) struct YdnsConfig {
    // The base URL of the YDNS API.
    #[config(default = "https://ydns.io/api/v1")]
    pub base_url: String,

    // The username to use for authentication.
    pub username: String,

    // The password to use for authentication.
    pub password: String,

    // The hosts to update.
    pub hosts: Vec<String>,

    // The time to wait between updates.
    #[config(default = 900)]
    pub wait_time: u64,

    // Whether to run as a daemon.
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
