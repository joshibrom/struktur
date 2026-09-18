//! Subcommand execution handlers for the CLI.

use anyhow::Result as AnyResult;
use struktur_core::{
    config::UserConfig,
    db::{
        self,
        models::{Job, JobStatus},
    },
    profile::Profile,
    storage::document::Document,
    template::{
        RenderableTemplate,
        cover_letter::{CoverLetterTemplateContext, plaintext::PlaintextTemplate},
        cv::plaintext::PlaintextCvTemplate,
    },
};

use crate::{
    cmd::JobAddArgs,
    helpers::{OutputContentType, OutputPath, open_file_in_editor},
    inspection,
};

pub type ActionResult = AnyResult<()>;

/// Initializes project storage by creating default `config.toml`, `profile.toml`, and template files.
///
/// # Errors
///
/// Returns an error if directory creation or file writing fails.
pub fn init() -> ActionResult {
    struktur_core::storage::init_storage()?;
    struktur_core::db::open()?;
    println!("Created project files.");
    Ok(())
}

/// Generates a tailored document using the specified preset, company, role, and output target.
///
/// # Errors
///
/// Returns an error if configuration or profile files cannot be loaded, the specified preset
/// is not found, or template rendering/writing fails.
pub fn generate(
    preset_name: String,
    company: String,
    role: String,
    date: String,
    output_path: OutputPath,
) -> ActionResult {
    let config = UserConfig::load()?;
    let profile = Profile::load()?;

    let preset = config
        .presets
        .get(&preset_name)
        .ok_or(anyhow::anyhow!("Unknown preset: {preset_name}"))?;

    let context = CoverLetterTemplateContext::new(role, company, date, profile, preset, &config)?;

    let content = PlaintextTemplate::render(&context)?;

    output_path.output(content, OutputContentType::CoverLetter)
}

/// Lists all configured presets formatted as a terminal table.
pub fn list_presets() -> ActionResult {
    let config = UserConfig::load()?;

    let table = inspection::listing::presets::list_presets_as_table(&config);

    println!("{table}");

    Ok(())
}

/// Lists all configured accomplishment bullets, optionally filtered by tag.
pub fn list_bullets(tag_filter: Option<String>) -> ActionResult {
    let config = UserConfig::load()?;

    let table = inspection::listing::bullets::list_bullets_as_table(&config, tag_filter);

    println!("{table}");

    Ok(())
}

/// Displays the filesystem paths and existence status of all project files.
pub fn get_status() -> ActionResult {
    inspection::status::check()
        .into_iter()
        .for_each(|check| println!("{check}"));

    Ok(())
}

/// Validates the format, syntax, and references of configuration, profile, and template files.
///
/// # Errors
///
/// Returns an error if one or more project files fail validation.
pub fn validate() -> ActionResult {
    let checks = inspection::validate::check();
    let has_errors = checks.iter().any(|check| !check.is_valid());

    for check in &checks {
        println!("{check}");
    }

    if has_errors {
        anyhow::bail!("One or more project files failed validation.");
    }

    Ok(())
}

/// Loads and displays the candidate profile in a formatted terminal view.
///
/// # Errors
///
/// Returns an error if the user profile cannot be loaded or rendered.
pub fn show_profile(as_json: bool) -> ActionResult {
    let profile = Profile::load()?;

    let output = if as_json {
        serde_json::to_string_pretty(&profile)?
    } else {
        inspection::profile::show_profile(profile)?
    };
    println!("{output}");

    Ok(())
}

/// Opens `config.toml` in the user's default text editor.
///
/// # Errors
///
/// Returns an error if the configuration path cannot be resolved or the editor fails to launch.
pub fn edit_config() -> ActionResult {
    edit_document::<UserConfig>()
}

/// Opens `profile.toml` in the user's default text editor.
///
/// # Errors
///
/// Returns an error if the profile path cannot be resolved or the editor fails to launch.
pub fn edit_profile() -> ActionResult {
    edit_document::<Profile>()
}

/// Opens the cover letter template in the user's default text editor.
///
/// # Errors
///
/// Returns an error if the template path cannot be resolved or the editor fails to launch.
pub fn edit_cover_letter_template() -> ActionResult {
    edit_template::<PlaintextTemplate>()
}

/// Opens the CV template in the user's default text editor.
///
/// # Errors
///
/// Returns an error if the template path cannot be resolved or the editor fails to launch.
pub fn edit_cv_template() -> ActionResult {
    edit_template::<PlaintextCvTemplate>()
}

fn edit_document<D: Document>() -> ActionResult {
    let path = D::get_path()?;
    Ok(open_file_in_editor(&path)?)
}

fn edit_template<T: RenderableTemplate>() -> ActionResult {
    let path = T::get_path()?;
    Ok(open_file_in_editor(&path)?)
}

/// Adds a new job application record to the local SQLite database.
///
/// # Errors
///
/// Returns an error if the database cannot be opened or if the database insertion transaction fails.
pub fn add_job(args: JobAddArgs) -> ActionResult {
    let mut job = Job::new(args.company, args.role);

    if let Some(status) = args.status {
        if status == JobStatus::Applied {
            job = job.with_apply_now();
        } else {
            job.status = status;
        }
    }

    job.location = args.location;
    job.salary_range = args.salary;
    job.job_url = args.url;
    job.notes = args.notes;
    job.contact_name = args.contact_name;
    job.contact_email = args.contact_email;

    let mut conn = db::open()?;
    let id =
        db::actions::within_transaction(&mut conn, |conn| db::actions::insert_job(conn, &job))?;

    println!("Added {} at {} (ID: {})", job.role, job.company, id);

    Ok(())
}

/// Lists tracked job applications formatted as a terminal table, optionally filtered by status.
///
/// # Errors
///
/// Returns an error if the database cannot be opened or if querying jobs fails.
pub fn list_jobs(status_filter: Option<JobStatus>) -> ActionResult {
    let conn = db::open()?;
    let jobs = db::actions::get_jobs(&conn, status_filter)?;

    let table = inspection::listing::db_models::list_jobs_as_table(jobs.as_slice());
    println!("{table}");

    Ok(())
}
