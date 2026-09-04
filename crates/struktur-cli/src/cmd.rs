//! Command-line argument definitions and parser configuration.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

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

    #[command(subcommand)]
    Profile(ProfileCommand),
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

#[derive(Subcommand, Debug)]
pub enum ProfileCommand {
    #[command()]
    Show,
}
