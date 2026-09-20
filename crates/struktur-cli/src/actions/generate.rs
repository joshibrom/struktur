use struktur_core::{
    config::UserConfig,
    db::{self, models::Rendering},
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
/// If `job_id` is specified, the generated document snapshot is saved to the database.
///
/// # Errors
///
/// Returns an error if configuration or profile files cannot be loaded, the specified preset
/// is not found, template rendering/writing fails, or database insertion fails.
pub fn generate(
    preset_name: String,
    company: String,
    role: String,
    date: String,
    output_path: OutputPath,
    job_id: Option<i64>,
) -> ActionResult {
    let config = UserConfig::load()?;
    let profile = Profile::load()?;

    let preset = config
        .presets
        .get(&preset_name)
        .ok_or(anyhow::anyhow!("Unknown preset: {preset_name}"))?;

    let context = CoverLetterTemplateContext::new(role, company, date, profile, preset, &config)?;

    let content = PlaintextTemplate::render(&context)?;

    output_path.output(&content, OutputContentType::CoverLetter)?;

    if let Some(id) = job_id {
        let rendering = Rendering::new(
            id,
            PlaintextTemplate::get_archetype(),
            "plaintext",
            &content,
            &context,
        )
        .with_preset(&preset_name);

        let conn = db::open()?;
        let rendering_id = db::actions::insert_rendering(&conn, &rendering)?;
        println!("Saved document snapshot [#{rendering_id}] linked to job [#{id}].");
    }

    Ok(())
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
