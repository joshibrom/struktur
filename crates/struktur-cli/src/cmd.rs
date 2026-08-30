use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Generate {
        #[arg(long)]
        preset: String,
        #[arg(long)]
        company: String,
        #[arg(long)]
        role: String,
        #[arg(long)]
        date: Option<String>,

        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        clipboard: bool,
    },
}
