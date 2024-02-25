use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, Config as SimplelogConfig, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};
use std::fs::OpenOptions;

pub fn setup(logfile: &str) {
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        LevelFilter::Info,
        SimplelogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];
    if !logfile.is_empty() {
        let logfile = WriteLogger::new(
            LevelFilter::Warn,
            SimplelogConfig::default(),
            OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(&logfile)
                .unwrap(),
        );
        loggers.push(logfile);
    }
    CombinedLogger::init(loggers).unwrap();
}
