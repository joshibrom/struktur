use struktur_core::{profile::Profile, storage::document::Document};

use crate::inspection;

use super::{ActionResult, edit_document};

/// Loads and displays the candidate profile in a formatted terminal view.
///
/// # Errors
///
/// Returns an error if the user profile cannot be loaded or rendered.
pub fn show(as_json: bool) -> ActionResult {
    let profile = Profile::load()?;

    let output = if as_json {
        serde_json::to_string_pretty(&profile)?
    } else {
        inspection::profile::show_profile(profile)?
    };
    println!("{output}");

    Ok(())
}

/// Opens `profile.toml` in the user's default text editor.
///
/// # Errors
///
/// Returns an error if the profile path cannot be resolved or the editor fails to launch.
pub fn edit() -> ActionResult {
    edit_document::<Profile>()
}
