//! Command-line interface entrypoint for `struktur`.

use clap::Parser;

use crate::cmd::{Cli, Commands, ListCommand};

mod actions;
mod cmd;
mod helpers;
mod listing;

/// Dispatches parsed CLI commands to their respective action handlers.
///
/// # Errors
///
/// Returns an error if the executed command action fails.
pub fn run(cli: Cli) -> anyhow::Result<()> {
    match &cli.command {
        Commands::Init => actions::init(),
        Commands::Generate {
            preset,
            company,
            role,
            date,
            output,
            clipboard,
        } => actions::generate(
            preset.clone(),
            company.clone(),
            role.clone(),
            date.clone().unwrap_or(helpers::today_as_string()),
            helpers::OutputPath::from_cmd_args(output, clipboard),
        ),
        Commands::List(lc) => match lc {
            ListCommand::Presets => actions::list_presets(),
        },
    }
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
