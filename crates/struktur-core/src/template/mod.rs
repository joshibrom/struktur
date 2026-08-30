//! Template rendering and variable interpolation subsystem.
//!
//! This module handles context resolution, pre-rendering of role/company variables
//! within preset hooks, and compiling documents via the [`RenderableTemplate`] trait.

use std::io::Write;

use serde::Serialize;
use tera::Tera;

use crate::{
    config::{Bullet, Preset, UserConfig},
    profile::Profile,
};

pub mod plaintext;

/// Context data passed into template engines for document rendering.
#[derive(Serialize, Debug, Clone)]
pub struct TemplateContext {
    /// Target job role or position title (e.g. "Senior Backend Engineer").
    pub role: String,
    /// Target company or organization name (e.g. "Acme Corp").
    pub company: String,
    /// Date of application (e.g. "August 29, 2026").
    pub date: String,

    /// Resolved accomplishment bullet points for the selected preset.
    pub bullets: Vec<Bullet>,
    /// Pre-rendered opening hook paragraph with interpolated role and company.
    pub opening_hook: String,
    /// Pre-rendered closing hook paragraph with interpolated role and company.
    pub closing_hook: String,

    /// Master candidate profile containing contact info, education, and experience.
    pub profile: Profile,
}

impl TemplateContext {
    /// Constructs and pre-renders a new `TemplateContext` from the given parameters.
    ///
    /// The preset's `opening_hook` and `closing_hook` template strings are evaluated
    /// and pre-rendered using the provided `role`, `company`, `date`, `profile`, and `bullets`.
    ///
    /// # Errors
    ///
    /// Returns a [`TemplateError`] if pre-rendering either hook string fails.
    pub fn new(
        role: String,
        company: String,
        date: String,
        profile: Profile,
        preset: &Preset,
        config: &UserConfig,
    ) -> Result<Self, TemplateError> {
        let bullets = preset
            .default_bullets
            .iter()
            .filter_map(|id| config.bullets.get(id))
            .cloned()
            .collect::<Vec<_>>();

        let opening_hook = Self::pre_render(
            &preset.opening_hook,
            &role,
            &company,
            &date,
            &profile,
            &bullets,
        )?;

        let closing_hook = Self::pre_render(
            &preset.closing_hook,
            &role,
            &company,
            &date,
            &profile,
            &bullets,
        )?;

        Ok(Self {
            role,
            company,
            date,
            bullets,
            profile,
            opening_hook,
            closing_hook,
        })
    }

    /// Evaluates a template snippet with the current context variables.
    fn pre_render(
        item: &str,
        role: &str,
        company: &str,
        date: &str,
        profile: &Profile,
        bullets: &[Bullet],
    ) -> Result<String, TemplateError> {
        let mut tera = Tera::default();
        tera.autoescape_on(Vec::<&str>::new());

        let mut ctx = tera::Context::new();
        ctx.insert("role", role);
        ctx.insert("company", company);
        ctx.insert("date", date);
        ctx.insert("profile", profile);
        ctx.insert("bullets", bullets);

        tera.render_str(item, &ctx, false)
            .map_err(TemplateError::TemplateRenderError)
    }
}

/// Errors that can occur during template loading, parsing, serialization, or rendering.
#[derive(thiserror::Error, Debug)]
pub enum TemplateError {
    /// An I/O error occurred while reading or writing a template file on disk.
    #[error("Failed to load template: {0}")]
    TemplateIoError(#[from] std::io::Error),

    /// A Tera syntax error occurred while registering or parsing the template.
    #[error("Failed to parse template: {0}")]
    TemplateAddError(tera::Error),

    /// A Tera execution error occurred while rendering the template.
    #[error("Failed to render template: {0}")]
    TemplateRenderError(tera::Error),

    /// Serialization of the template context to a Tera value failed.
    #[error("Failed to serialize template context: {0}")]
    TemplateContextSerializationError(tera::Error),
}

/// A document template that can be rendered using a [`TemplateContext`].
///
/// Implementations define a file name and an embedded default fallback template.
/// When rendering, the implementation first checks for a user-customized template
/// file on disk (in `~/.config/struktur/templates/<file_name>`), falling back to
/// the embedded default if no custom template exists.
pub trait RenderableTemplate {
    /// The template file name on disk (e.g. `plaintext.tera`).
    fn file_name() -> &'static str;

    /// The embedded default template used when no user file exists on disk.
    fn get_default_template() -> &'static str;

    /// Resolves the filesystem path to the template in the user configuration directory.
    ///
    /// # Errors
    ///
    /// Returns a [`std::io::Error`] if the configuration directory cannot be determined.
    fn get_path() -> Result<std::path::PathBuf, std::io::Error> {
        let path = crate::storage::get_project_dirs()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find project directories",
                )
            })?
            .config_dir()
            .to_owned();
        Ok(path.join("templates").join(Self::file_name()))
    }

    /// Checks whether the custom template file exists on disk.
    ///
    /// # Errors
    ///
    /// Returns a [`TemplateError`] if path resolution fails.
    fn exists() -> Result<bool, TemplateError> {
        Ok(Self::get_path()?.exists())
    }

    /// Loads the custom template string from disk.
    ///
    /// # Errors
    ///
    /// Returns a [`std::io::Error`] if the file cannot be read.
    fn load() -> Result<String, std::io::Error> {
        let path = Self::get_path()?;
        std::fs::read_to_string(path)
    }

    /// Retrieves the template string to use, preferring a custom file on disk
    /// and falling back to [`Self::get_default_template()`].
    fn get_template() -> String {
        match Self::load() {
            Ok(templ) => templ,
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("Warning: Failed to load custom template: {err}");
                }
                Self::get_default_template().to_string()
            }
        }
    }

    /// Renders the template using the provided [`TemplateContext`].
    ///
    /// # Errors
    ///
    /// Returns a [`TemplateError`] if template registration, context serialization,
    /// or rendering fails.
    fn render(context: &TemplateContext) -> Result<String, TemplateError> {
        let template = Self::get_template();

        let mut tera = Tera::new();
        tera.autoescape_on(Vec::<&str>::new());
        tera.add_raw_template("main", &template)
            .map_err(TemplateError::TemplateAddError)?;

        let tera_ctx = tera::Context::from_serialize(context)
            .map_err(TemplateError::TemplateContextSerializationError)?;

        let rendering = tera
            .render("main", &tera_ctx)
            .map_err(TemplateError::TemplateRenderError)?;

        Ok(rendering.trim().to_string())
    }

    /// Writes the embedded default template to disk if it does not already exist.
    ///
    /// # Errors
    ///
    /// Returns a [`TemplateError`] if parent directory creation or file writing fails.
    fn write_default_template() -> Result<(), TemplateError> {
        let path = Self::get_path()?;
        if Self::exists()? {
            return Ok(());
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::File::create(path)?;
        file.write_all(Self::get_default_template().as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{plaintext::PlaintextTemplate, *};

    #[test]
    fn test_template_context_new_and_prerender() {
        let profile = Profile::default();
        let config = UserConfig::default();
        let preset = config.presets.get("backend").expect("preset should exist");

        let context = TemplateContext::new(
            "Principal Engineer".into(),
            "Acme Corp".into(),
            "2026-08-29".into(),
            profile.clone(),
            preset,
            &config,
        )
        .expect("context creation should succeed");

        assert_eq!(context.role, "Principal Engineer");
        assert_eq!(context.company, "Acme Corp");
        assert_eq!(context.date, "2026-08-29");
        assert_eq!(context.profile.name, profile.name);
        assert!(!context.bullets.is_empty());

        // Verify pre-rendered hooks replaced variables
        assert!(context.opening_hook.contains("Principal Engineer"));
        assert!(context.opening_hook.contains("Acme Corp"));
        assert!(!context.opening_hook.contains("{{ role }}"));
        assert!(!context.opening_hook.contains("{{ company }}"));

        assert!(context.closing_hook.contains("Acme Corp"));
        assert!(!context.closing_hook.contains("{{ company }}"));
    }

    #[test]
    fn test_plaintext_template_render() {
        let profile = Profile::default();
        let config = UserConfig::default();
        let preset = config.presets.get("backend").expect("preset should exist");

        let context = TemplateContext::new(
            "Staff Backend Engineer".into(),
            "Stripe".into(),
            "August 29, 2026".into(),
            profile.clone(),
            preset,
            &config,
        )
        .expect("context creation should succeed");

        let rendered = PlaintextTemplate::render(&context).expect("rendering should succeed");

        assert!(rendered.contains(&profile.name));
        assert!(rendered.contains(&profile.email));
        assert!(rendered.contains("August 29, 2026"));
        assert!(rendered.contains("Regarding: Staff Backend Engineer Position"));
        assert!(rendered.contains("Stripe"));
        assert!(rendered.contains("Key Highlights:"));

        for bullet in &context.bullets {
            assert!(rendered.contains(&bullet.text));
        }
    }

    #[test]
    fn test_default_template_fallback() {
        let fallback = PlaintextTemplate::get_default_template();
        assert!(fallback.contains("{{ profile.name }}"));
        assert!(fallback.contains("{{ opening_hook }}"));
        assert!(fallback.contains("{{ closing_hook }}"));
    }
}
