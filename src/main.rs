use clap::{Parser, Subcommand};
use cmd::filter;
use config::Config;

mod alfred;
mod cache;
mod cmd;
mod config;
mod github;

#[derive(Debug, Parser)]
#[clap(name = "ghz")]
#[clap(about = "Fuzzy find git repositories.")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Refresh local cache from Github
    Refresh,
    /// Lists all repositories matching the filter string
    Filter(filter::Opts),
}

fn main() -> anyhow::Result<()> {
    let cfg = Config::load()?;
    let args = Cli::parse();

    match args.command {
        Commands::Refresh => {
            cmd::refresh::run(&cfg)?;
        }
        Commands::Filter(opts) => {
            cmd::filter::run(&cfg, opts)?;
        }
    }

    Ok(())
}
