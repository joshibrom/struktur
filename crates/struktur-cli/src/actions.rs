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
    helpers::{OutputContentType, OutputPath},
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
pub fn list_bullets(tag_filter: &Option<String>) -> ActionResult {
    let config = UserConfig::load()?;

    let table = inspection::listing::bullets::list_bullets_as_table(&config, tag_filter.to_owned());

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

pub fn show_profile() -> ActionResult {
    let profile = Profile::load()?;

    let cv = inspection::profile::show_profile(profile)?;
    println!("{cv}");

    Ok(())
}
