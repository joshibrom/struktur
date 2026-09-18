//! Command-line interface entrypoint for `struktur`.

use clap::Parser;

use crate::cmd::{
    Cli, Commands, EditCommand, EditTemplateCommand, JobCommand, ListCommand, ProfileCommand,
};

mod actions;
mod cmd;
mod helpers;
mod inspection;

/// Dispatches parsed CLI commands to their respective action handlers.
///
/// # Errors
///
/// Returns an error if the executed command action fails.
pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Init => actions::init(),
        Commands::Generate {
            preset,
            company,
            role,
            date,
            output,
            clipboard,
        } => actions::generate(
            preset,
            company,
            role,
            date.unwrap_or(helpers::today_as_string()),
            helpers::OutputPath::from_cmd_args(output, clipboard),
        ),
        Commands::Status => actions::get_status(),
        Commands::Validate => actions::validate(),
        Commands::List(lc) => match lc {
            ListCommand::Presets => actions::list_presets(),
            ListCommand::Bullets { tag } => actions::list_bullets(tag),
        },
        Commands::Profile(pc) => match pc {
            ProfileCommand::Show { json } => actions::show_profile(json),
        },
        Commands::Edit(ec) => match ec {
            EditCommand::Config => actions::edit_config(),
            EditCommand::Profile => actions::edit_profile(),
            EditCommand::Template(template) => match template {
                EditTemplateCommand::CoverLetter => actions::edit_cover_letter_template(),
                EditTemplateCommand::Cv => actions::edit_cv_template(),
            },
        },
        Commands::Job(jc) => match jc {
            JobCommand::Add(args) => actions::add_job(*args),
            JobCommand::List { status } => actions::list_jobs(status),
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
