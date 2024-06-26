use std::{path::PathBuf, process::exit, time::Duration};

use ydns::{
    file_utils::{get_ip_from_file, write_ip_to_file},
    ydns_client::{get_current_ip, update_host},
};

use crate::config;

pub(crate) async fn run(
    config: &config::YdnsConfig,
    last_ip_file: &PathBuf,
    force: bool,
    dry_run: bool,
) {
    let daemon = config.daemon;
    let mut has_run = false;
    let mut last_ip = "".to_string();
    loop {
        if has_run {
            if dry_run {
                println!("Dry run debug: waiting for {} seconds", config.wait_time);
            }
            tokio::time::sleep(Duration::from_secs(config.wait_time)).await;
        }

        let current_ip = match get_current_ip(&config.base_url).await {
            Ok(ip) => ip,
            Err(e) => {
                eprintln!("Could not get current IP: {}", e);
                if !daemon {
                    exit(1)
                }
                continue;
            }
        };

        if !daemon {
            last_ip = match get_ip_from_file(&last_ip_file) {
                Ok(ip) => ip,
                Err(e) => {
                    eprintln!("Could not get IP from file: {}", e);
                    if !daemon {
                        exit(1)
                    }
                    continue;
                }
            };
        }

        if last_ip == current_ip && !force {
            if !daemon {
                exit(0)
            }
            continue;
        }

        for host in config.hosts.iter() {
            if dry_run {
                println!("Would update host {} with IP address {}", host, current_ip);
            } else {
                if let Err(e) = update_host(
                    &config.base_url,
                    &config.username,
                    &config.password,
                    &host,
                    &current_ip,
                )
                .await
                {
                    eprintln!("Could not update host {}: {}", host, e);
                    if !daemon {
                        exit(1)
                    }
                    continue;
                }
            }
        }

        if !daemon {
            if let Err(e) = write_ip_to_file(&last_ip_file, &current_ip) {
                eprintln!("Could not write IP to file: {}", e);
                exit(1)
            }
        }

        println!("Successfully updated hosts with new IP address: {current_ip}");

        if !daemon {
            exit(0)
        }

        last_ip = current_ip.clone();
        has_run = true;
    }
}
