use std::{error::Error, path::PathBuf};

pub fn get_ip_from_file(last_ip_file: &PathBuf) -> Result<String, Box<dyn Error>> {
    let ip = match std::fs::read_to_string(last_ip_file) {
        Ok(ip) => ip,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                // File does not exist, ignore error and return empty string
                return Ok("".to_string());
            }
            _ => {
                return Err(e.into());
            }
        },
    };
    Ok(ip)
}

pub fn write_ip_to_file(last_ip_file: &PathBuf, ip: &str) -> Result<(), Box<dyn Error>> {
    std::fs::write(last_ip_file, ip)?;
    Ok(())
}
