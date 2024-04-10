use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::exit;

mod commands;
mod config;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Get current ip.
    Ip,

    /// Update host(s) with current ip.
    Update {
        /// The host(s) to update
        #[arg(required = true, long, short = 'H')]
        host: Vec<String>,

        /// Last IP file
        #[arg(long, default_value = "/tmp/ydns_last_ip")]
        last_ip_file: PathBuf,

        /// Force update
        #[arg(action, long, short = 'f')]
        force: bool,

        /// Use as daemon
        #[arg(action, long, short = 'd')]
        daemon: bool,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct GlobalOpts {
    /// The configuration file(s)
    #[arg(long, short, default_values = ["ydns.yaml", "/etc/ydns/ydns.yaml"])]
    config: Vec<PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = match config::load(&cli.global_opts.config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read the configuration: {}", e);
            exit(1)
        }
    };

    match cli.command {
        Command::Ip => commands::ip::run(&config).await,
        Command::Update {
            host,
            last_ip_file,
            force,
            daemon,
        } => commands::update::run(&config, host, &last_ip_file, force, daemon).await,
    }
}
