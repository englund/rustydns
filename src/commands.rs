use log::{error, info};
use std::process::exit;

use ydns::{get_current_ip, update_host};

use crate::config;

pub(crate) async fn get_ip(config: &config::YdnsConfig) {
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

pub(crate) async fn update(config: &config::YdnsConfig, host: Vec<String>) {
    let current_ip = match get_current_ip(&config.base_url).await {
        Ok(ip) => ip,
        Err(e) => {
            error!("Could not get current IP: {}", e);
            exit(1)
        }
    };

    info!("Current IP: {current_ip}");

    for host in host.iter() {
        info!("Host: {host}");
        if let Err(e) = update_host(
            &config.base_url,
            &config.username,
            &config.password,
            &host,
            &current_ip,
        )
        .await
        {
            error!("Could not update host {}: {}", host, e);
            exit(1)
        }
    }
}
