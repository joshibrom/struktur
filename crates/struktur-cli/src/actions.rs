//! Subcommand execution handlers for the CLI.

use anyhow::Result as AnyResult;
use struktur_core::{
    config::UserConfig,
    profile::Profile,
    storage::document::Document,
    template::{
        RenderableTemplate,
        cover_letter::{CoverLetterTemplateContext, plaintext::PlaintextTemplate},
    },
};

use crate::{
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
    let config_path = UserConfig::get_path()?;
    let code = open_file_in_editor(&config_path)?;

    if code != 0 {
        eprintln!("Warning: Non-Zero exit code received: {code}");
    }

    Ok(())
}

/// Opens `profile.toml` in the user's default text editor.
///
/// # Errors
///
/// Returns an error if the profile path cannot be resolved or the editor fails to launch.
pub fn edit_profile() -> ActionResult {
    let profile_path = Profile::get_path()?;
    let code = open_file_in_editor(&profile_path)?;

    if code != 0 {
        eprintln!("Warning: Non-Zero exit code received: {code}");
    }

    Ok(())
}
