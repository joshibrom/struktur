use struktur_core::{config::UserConfig, storage::document::Document};

use crate::inspection;

use super::{ActionResult, edit_document};

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

/// Opens `config.toml` in the user's default text editor.
///
/// # Errors
///
/// Returns an error if the configuration path cannot be resolved or the editor fails to launch.
pub fn edit_config() -> ActionResult {
    edit_document::<UserConfig>()
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
