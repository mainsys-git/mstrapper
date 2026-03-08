use crate::generators::vite::ViteArgs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dev")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Vite(ViteArgs),
}