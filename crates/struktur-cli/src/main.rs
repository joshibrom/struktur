//! Command-line interface entrypoint for `struktur`.

use clap::Parser;

use crate::cmd::{
    Cli, Commands, EditCommand, EditTemplateCommand, JobCommand, JobUpdateCommand, ListCommand,
    ProfileCommand,
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
        Commands::Init => actions::system::init(),
        Commands::Generate {
            preset,
            company,
            role,
            date,
            output,
            clipboard,
        } => actions::generate::generate(
            preset,
            company,
            role,
            date.unwrap_or(helpers::today_as_string()),
            helpers::OutputPath::from_cmd_args(output, clipboard),
        ),
        Commands::Status => actions::system::get_status(),
        Commands::Validate => actions::system::validate(),
        Commands::List(lc) => match lc {
            ListCommand::Presets => actions::system::list_presets(),
            ListCommand::Bullets { tag } => actions::system::list_bullets(tag),
        },
        Commands::Profile(pc) => match pc {
            ProfileCommand::Show { json } => actions::profile::show(json),
        },
        Commands::Edit(ec) => match ec {
            EditCommand::Config => actions::system::edit_config(),
            EditCommand::Profile => actions::profile::edit(),
            EditCommand::Template(template) => match template {
                EditTemplateCommand::CoverLetter => actions::generate::edit_cover_letter_template(),
                EditTemplateCommand::Cv => actions::generate::edit_cv_template(),
            },
        },
        Commands::Job(jc) => match jc {
            JobCommand::Add(args) => actions::job::add(*args),
            JobCommand::List { status } => actions::job::list_all(status),
            JobCommand::Show { job_id } => actions::job::show(job_id),
            JobCommand::Update { job_id, target } => match target {
                JobUpdateCommand::Company { name } => actions::job::update_company(job_id, name),
                JobUpdateCommand::Role { name } => actions::job::update_role(job_id, name),
                JobUpdateCommand::Status {
                    status,
                    description,
                } => actions::job::update_status(job_id, status, description),
                JobUpdateCommand::Location { location } => {
                    actions::job::update_location(job_id, location)
                }
                JobUpdateCommand::Salary { salary } => actions::job::update_salary(job_id, salary),
                JobUpdateCommand::Url { url } => actions::job::update_url(job_id, url),
                JobUpdateCommand::Notes { notes } => actions::job::update_notes(job_id, notes),
                JobUpdateCommand::Contact { name, email } => {
                    actions::job::update_contact(job_id, name, email)
                }
            },
            JobCommand::Rm { job_id } => actions::job::delete(job_id),
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
