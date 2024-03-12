use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, Config as SimplelogConfig, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};
use std::{error::Error, fs::OpenOptions};

use crate::GlobalOpts;

pub fn setup(opts: &GlobalOpts) -> Result<(), Box<dyn Error>> {
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        LevelFilter::Info,
        SimplelogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];
    match &opts.logfile {
        Some(p) => {
            let logfile = WriteLogger::new(
                LevelFilter::Warn,
                SimplelogConfig::default(),
                OpenOptions::new().create_new(true).append(true).open(p)?,
            );
            loggers.push(logfile);
        }
        None => {}
    };
    CombinedLogger::init(loggers)?;

    Ok(())
}
