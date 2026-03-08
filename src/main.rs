use crate::cli::Commands;
use clap::Parser;
use anyhow::Result;

mod cli;
mod generators;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        Commands::Vite(args) => {
            generators::vite::run(args)?;
        }
    }
    Ok(())
}