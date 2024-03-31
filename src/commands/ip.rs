use log::{error, info};
use std::process::exit;

use ydns::ydns_client::get_current_ip;

use crate::config;

pub(crate) async fn run(config: &config::YdnsConfig) {
    match get_current_ip(&config.base_url).await {
        Ok(ip) => {
            info!("{ip}");
        }
        Err(e) => {
            error!("Could not get current IP: {}", e);
            exit(1)
        }
    };
}
