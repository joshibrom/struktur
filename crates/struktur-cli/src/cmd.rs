//! Command-line argument definitions and parser configuration.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use struktur_core::db::models::JobStatus;

/// Command-line parser for `struktur`.
#[derive(Parser, Debug)]
#[command(
    name = "struktur",
    version,
    about = "A local-first CLI for generating tailored job application materials",
    long_about = "struktur is a local-first workstation for managing structured candidate profiles, \
                  reusable accomplishment bullets, and generating tailored cover letters and application materials."
)]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize default configuration (`config.toml`), user profile (`profile.toml`), and template files.
    #[command(about = "Initialize default configuration, profile, and template files")]
    Init,

    /// Generate a tailored cover letter from a preset and candidate profile.
    #[command(about = "Generate a tailored cover letter")]
    Generate {
        /// ID of the preset to use (e.g. "backend").
        #[arg(long, help = "ID of the role preset to use")]
        preset: String,

        /// Target company or organization name.
        #[arg(long, help = "Target company or organization name")]
        company: String,

        /// Target job title or role position.
        #[arg(long, help = "Target job title or role position")]
        role: String,

        /// Application date (defaults to today's date if omitted).
        #[arg(long, help = "Application date (defaults to current date if omitted)")]
        date: Option<String>,

        /// Optional file path to write the generated document to.
        #[arg(short, long, help = "Write output to the specified file path")]
        output: Option<PathBuf>,

        /// Copy the generated document directly to the system clipboard.
        #[arg(short, long, help = "Copy output to the system clipboard")]
        clipboard: bool,
    },

    /// List configured resources (presets, bullets, etc.)
    #[command(subcommand, about = "List available presets or bullets")]
    List(ListCommand),

    /// Display filesystem paths and existence status for project files.
    #[command(about = "Display project file paths and status")]
    Status,

    /// Validate configuration, profile, and template files for syntax or reference errors.
    #[command(about = "Validate configuration, profile, and template files")]
    Validate,

    /// Manage and inspect the candidate master profile.
    #[command(subcommand, about = "Manage and inspect user profile")]
    Profile(ProfileCommand),

    /// Open project configuration and profile files in an editor.
    #[command(subcommand, about = "Open project configuration or profile in $EDITOR")]
    Edit(EditCommand),

    /// Manage and track job applications.
    #[command(subcommand, about = "Manage and track job applications")]
    Job(JobCommand),
}

#[derive(Subcommand, Debug)]
pub enum ListCommand {
    /// List all configured bullets in a table
    #[command(about = "List all configured bullets")]
    Bullets {
        /// Optionally filter bullets by a tag name
        #[arg(short, long, help = "Filter by bullets with given tag")]
        tag: Option<String>,
    },
    /// List all configured role presets in a table
    #[command(about = "List all configured role presets")]
    Presets,
}

/// Available profile subcommands.
#[derive(Subcommand, Debug)]
pub enum ProfileCommand {
    /// Display a formatted summary of the candidate profile.
    #[command(about = "Display formatted candidate profile summary")]
    Show {
        /// Output profile information in JSON format
        #[arg(short, long, help = "Output profile information in JSON format")]
        json: bool,
    },
}

/// Available edit subcommands specifying which file to open in an editor.
#[derive(Subcommand, Debug)]
pub enum EditCommand {
    /// Open config.toml in your default editor.
    #[command(about = "Open config.toml in your default editor")]
    Config,

    /// Open profile.toml in your default editor.
    #[command(about = "Open profile.toml in your default editor")]
    Profile,

    /// Open specified template in your default editor.
    #[command(subcommand, about = "Open a specified template in your default editor")]
    Template(EditTemplateCommand),
}

/// Available template targets for editing in an editor.
#[derive(Subcommand, Debug)]
pub enum EditTemplateCommand {
    /// Open the cover letter template in your default editor.
    #[command(about = "Open the cover letter template in your default editor")]
    CoverLetter,

    /// Open the CV template in your default editor.
    #[command(about = "Open the CV template in your default editor")]
    Cv,
}

/// Available job tracking subcommands.
#[derive(Subcommand, Debug)]
pub enum JobCommand {
    /// Add a new job application to track.
    #[command(about = "Add a new job application to track")]
    Add(Box<JobAddArgs>),

    #[command(alias = "gen")]
    Generate {
        #[arg()]
        job_id: i64,

        /// ID of the preset to use (e.g. "backend").
        #[arg(long, help = "ID of the role preset to use")]
        preset: String,

        /// Application date (defaults to today's date if omitted).
        #[arg(long, help = "Application date (defaults to current date if omitted)")]
        date: Option<String>,

        /// Optional file path to write the generated document to.
        #[arg(short, long, help = "Write output to the specified file path")]
        output: Option<PathBuf>,

        /// Copy the generated document directly to the system clipboard.
        #[arg(short, long, help = "Copy output to the system clipboard")]
        clipboard: bool,
    },

    /// List job applications in a table.
    #[command(about = "List tracked job applications")]
    List {
        /// Filter applications by status
        #[arg(long, help = "Filter applications by status")]
        status: Option<JobStatus>,
    },

    /// Display detailed information, timeline, and documents for a job application.
    #[command(about = "Display detailed information and timeline for a job application")]
    Show {
        /// Database ID of the job to inspect.
        #[arg(help = "Database ID of the job to inspect")]
        job_id: i64,
    },

    /// Update an existing job application record.
    #[command(about = "Update an existing job application")]
    Update {
        /// Database ID of the job to update.
        #[arg(help = "Database ID of the job to update")]
        job_id: i64,

        /// The update operation to perform.
        #[command(subcommand)]
        target: JobUpdateCommand,
    },

    /// Delete an existing job application record and its associated history.
    #[command(alias = "delete", about = "Delete an existing job application record.")]
    Rm {
        #[arg(help = "Database ID of the job to delete")]
        job_id: i64,
    },
}

/// Command-line arguments for adding a new job application.
#[derive(clap::Args, Debug)]
pub struct JobAddArgs {
    /// Target company or organization name.
    #[arg(short, long, help = "Target company or organization name")]
    pub company: String,

    /// Target job title or role position.
    #[arg(short, long, help = "Target job title or role position")]
    pub role: String,

    /// Initial application status (defaults to 'saved' if omitted).
    #[arg(long, help = "Initial application status (e.g. 'saved', 'applied')")]
    pub status: Option<JobStatus>,

    /// Job location or work arrangement (e.g. 'Remote', 'New York, NY', 'Hybrid').
    #[arg(long, help = "Job location or work arrangement")]
    pub location: Option<String>,

    /// Target salary or compensation range.
    #[arg(long, help = "Salary or compensation range")]
    pub salary: Option<String>,

    /// URL to the job posting.
    #[arg(long, help = "URL to the job posting")]
    pub url: Option<String>,

    /// Notes or referral details.
    #[arg(long, help = "Notes or referral details")]
    pub notes: Option<String>,

    /// Primary recruiter, hiring manager, or referral contact name.
    #[arg(long, help = "Primary contact name")]
    pub contact_name: Option<String>,

    /// Primary contact email address.
    #[arg(long, help = "Primary contact email address")]
    pub contact_email: Option<String>,
}

/// Subcommands specifying which attribute of a job application to update.
#[derive(Subcommand, Debug)]
pub enum JobUpdateCommand {
    /// Update the company or organization name.
    #[command(about = "Update the company name")]
    Company {
        /// New company or organization name.
        #[arg(help = "New company name")]
        name: String,
    },

    /// Update the job role or position title.
    #[command(about = "Update the job role or position title")]
    Role {
        /// New job role or position title.
        #[arg(help = "New job role or title")]
        name: String,
    },

    /// Transition the application status and record a status change event.
    #[command(about = "Transition application status and record an event")]
    Status {
        /// New application status (e.g. 'saved', 'applied', 'interviewing', 'offer', 'rejected', 'withdrawn').
        #[arg(help = "New application status")]
        status: JobStatus,

        /// Optional note or rationale explaining the status transition.
        #[arg(
            short,
            long,
            help = "Optional note or rationale for the status transition"
        )]
        description: Option<String>,
    },

    /// Update the job location or work arrangement.
    #[command(about = "Update the job location or work arrangement")]
    Location {
        /// New job location or work arrangement (e.g. 'Remote', 'New York, NY', 'Hybrid').
        #[arg(help = "New job location or work arrangement")]
        location: String,
    },

    /// Update the target salary or compensation range.
    #[command(about = "Update the salary or compensation range")]
    Salary {
        /// New salary or compensation range (e.g. '$150k - $180k').
        #[arg(help = "New salary or compensation range")]
        salary: String,
    },

    /// Update the URL to the job posting.
    #[command(about = "Update the URL to the job posting")]
    Url {
        /// New URL to the job posting.
        #[arg(help = "New job posting URL")]
        url: String,
    },

    /// Update notes or referral details.
    #[command(about = "Update notes or referral details")]
    Notes {
        /// New notes or referral details.
        #[arg(help = "New notes or referral details")]
        notes: String,
    },

    /// Update primary recruiter, hiring manager, or referral contact information.
    #[command(about = "Update primary contact details")]
    Contact {
        /// Primary contact name.
        #[arg(short, long, help = "Primary recruiter or contact name")]
        name: Option<String>,

        /// Primary contact email address.
        #[arg(short, long, help = "Primary contact email address")]
        email: Option<String>,
    },
}
