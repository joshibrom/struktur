use clap::Parser;

use crate::cmd::{Cli, Commands};

mod actions;
mod cmd;

pub fn run(cli: Cli) {
    match &cli.command {
        Commands::Init => actions::init(),
        Commands::Generate {
            preset,
            company,
            role,
            date,
        } => actions::generate(preset.clone(), company.clone(), role.clone(), date.clone()),
    }
    .expect("Failed to run command")
}

fn main() {
    let cli = Cli::parse();
    run(cli);
}
