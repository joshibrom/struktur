use clap::Parser;

use crate::cmd::{Cli, Commands};

mod cmd;

pub fn run(cli: Cli) {
    match &cli.command {
        Commands::Init => {
            // TODO: Handle error better
            struktur_core::storage::ensure_project_files()
                .expect("project files should be creatable");
            println!("Created project files."); // TODO: Better logging
        }
    }
}

fn main() {
    let cli = Cli::parse();
    run(cli);
}
