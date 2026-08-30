use clap::Parser;

use crate::cmd::{Cli, Commands};

mod actions;
mod cmd;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match &cli.command {
        Commands::Init => actions::init(),
        Commands::Generate {
            preset,
            company,
            role,
            date,
        } => actions::generate(preset.clone(), company.clone(), role.clone(), date.clone()),
    }
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
