use struktur_core::{
    config::UserConfig,
    profile::Profile,
    storage::document::Document,
    template::{
        RenderableTemplate,
        cover_letter::{CoverLetterTemplateContext, plaintext::PlaintextTemplate},
        cv::plaintext::PlaintextCvTemplate,
    },
};

use crate::helpers::{OutputContentType, OutputPath};

use super::{ActionResult, edit_template};

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
