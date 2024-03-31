use log::{error, info};
use std::{path::PathBuf, process::exit};

use ydns::{
    file_utils::{get_ip_from_file, write_ip_to_file},
    ydns_client::{get_current_ip, update_host},
};

use crate::config;

pub async fn run(
    config: &config::YdnsConfig,
    host: Vec<String>,
    last_ip_file: &PathBuf,
    force: bool,
) {
    let current_ip = match get_current_ip(&config.base_url).await {
        Ok(ip) => ip,
        Err(e) => {
            error!("Could not get current IP: {}", e);
            exit(1)
        }
    };

    let last_ip = match get_ip_from_file(&last_ip_file) {
        Ok(ip) => ip,
        Err(e) => {
            error!("Could not get IP from file: {}", e);
            exit(1)
        }
    };

    info!("Last IP: {last_ip}");
    info!("Current IP: {current_ip}");

    if last_ip == current_ip && !force {
        info!("IP has not changed, exiting");
        exit(0)
    }

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

    if let Err(e) = write_ip_to_file(&last_ip_file, &current_ip) {
        error!("Could not write IP to file: {}", e);
        exit(1)
    }
}
